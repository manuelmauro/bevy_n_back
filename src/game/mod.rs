// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{
    despawn_screen,
    game::core::cue::SPACING,
    game::core::cue::{Cell, Pigment},
    game::core::NBack,
    menu::MenuState,
    GameState,
};
use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_kira_audio::{prelude::*, Audio};

pub mod core;

#[derive(Component, Deref, DerefMut)]
struct CellTimer(Timer);

pub struct GamePlugin;

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.15, 0.15, 0.15)))
            .add_plugins(EguiPlugin)
            .add_plugins(AudioPlugin)
            .insert_resource(NBack::default())
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                Update,
                (
                    timer_system,
                    answer_system,
                    cue_system.after(answer_system),
                    debug_ui,
                    exit_game_system,
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    // Add game's entities to our world
    // audio
    audio
        .play(asset_server.load("sounds/Cyberpunk-Moonlight-Sonata.ogg"))
        .looped();

    // Add walls
    let wall_color = Color::rgb(1.0, 1.0, 1.0);
    let wall_thickness = 8.0;
    let bounds = Vec2::new(240.0, 240.0);
    // left
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
                ..Default::default()
            },
            ..Default::default()
        },
        OnGameScreen,
    ));
    // right
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
                ..Default::default()
            },
            ..Default::default()
        },
        OnGameScreen,
    ));
    // bottom
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
                ..Default::default()
            },
            ..Default::default()
        },
        OnGameScreen,
    ));
    // top
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
                ..Default::default()
            },
            ..Default::default()
        },
        OnGameScreen,
    ));

    // Add cell
    let cell = Cell::None;
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: (&Pigment::A).into(),
                    custom_size: Some(Vec2::new(
                        (bounds.x - SPACING) / 3.0,
                        (bounds.x - SPACING) / 3.0,
                    )),
                    ..Default::default()
                },
                transform: Transform::from_translation((&cell).into()),
                ..Default::default()
            },
            OnGameScreen,
        ))
        .insert(cell)
        .insert(CellTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
}

/// Tick all the `Timer` components on entities within the scene using bevy's
/// `Time` resource to get the delta between each update.
fn timer_system(time: Res<Time>, mut query: Query<&mut CellTimer>) {
    for mut timer in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            info!("tick!")
        }
    }
}

/// Render cues.
fn cue_system(
    mut game: ResMut<NBack>,
    mut board_query: Query<(&Cell, &mut Transform, &mut Sprite, &CellTimer)>,
) {
    if let Ok((_, mut transform, mut sprite, timer)) = board_query.get_single_mut() {
        if timer.just_finished() {
            if let Some((new_cell, new_pigment)) = game.next() {
                info!("cue: {:?}", new_cell);
                transform.translation = (&new_cell).into();
                sprite.color = (&new_pigment).into();
            }
        }
    }
}

/// Record answers.
fn answer_system(
    mut game: ResMut<NBack>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&CellTimer>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        game.answer.w();
    }
    if keyboard_input.pressed(KeyCode::A) {
        game.answer.a();
    }
    if keyboard_input.pressed(KeyCode::S) {
        game.answer.s();
    }
    if keyboard_input.pressed(KeyCode::D) {
        game.answer.d();
    }

    if let Ok(timer) = query.get_single_mut() {
        if timer.just_finished() {
            game.check_answer();
            game.answer.reset();
            info!("reset answer");
        }
    }
}

/// Exit game.
fn exit_game_system(
    mut game: ResMut<NBack>,
    keyboard_input: Res<Input<KeyCode>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    audio: Res<Audio>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
        menu_state.set(MenuState::Main);
        audio.stop();
        game.restart();
    }
}

/// User interface.
fn debug_ui(
    mut camera: Query<(Entity, Option<&mut BloomSettings>), With<Camera>>,
    mut egui_context: EguiContexts,
    mut game: ResMut<NBack>,
) {
    let mut bloom_settings = camera.single_mut().1.unwrap();

    egui::Window::new("bloom")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut bloom_settings.intensity, 0.0..=1.0).text("intensity"));
            ui.add(
                egui::Slider::new(&mut bloom_settings.low_frequency_boost, 0.0..=1.0)
                    .text("low_frequency_boost"),
            );
            ui.add(
                egui::Slider::new(&mut bloom_settings.high_pass_frequency, 0.0..=1.0)
                    .text("high_pass_frequency"),
            );
            ui.add(
                egui::Slider::new(&mut bloom_settings.low_frequency_boost_curvature, 0.0..=1.0)
                    .text("low_frequency_boost_curvature"),
            );
        });

    egui::Window::new("debug")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(format!("n back: {}", game.cells.n_back()));
            ui.label(format!("correct: {}", game.score.correct()));
            ui.label(format!("wrong: {}", game.score.wrong()));
            ui.label(format!("F1 score: {}", game.score.f1_score()));
            ui.label(format!("{:?}", game.answer));

            if ui.button("Restart").clicked() {
                game.restart()
            }
        });
}
