// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_kira_audio::prelude::*;
use bevy_n_back::{
    constant::SPACING,
    nback::cue::{Cell, Pigment},
    nback::NBack,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum Label {
    ScoreCheck,
}

#[derive(Component, Deref, DerefMut)]
struct CellTimer(Timer);

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "bevy_n_back".to_string(),
        width: 360.,
        height: 640.,
        mode: WindowMode::Windowed,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_plugin(AudioPlugin)
    .insert_resource(NBack::default())
    .insert_resource(ClearColor(Color::rgb(0.15, 0.15, 0.15)))
    .add_startup_system(setup)
    .add_system(timer_system)
    .add_system(score_system.label(Label::ScoreCheck))
    .add_system(cue_system.after(Label::ScoreCheck))
    .add_system(answer_system.after(Label::ScoreCheck))
    .add_system(debug_ui);

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    // Add game's entities to our world
    // audio
    audio
        .play(asset_server.load("sounds/Cyberpunk-Moonlight-Sonata.ogg"))
        .looped();

    // cameras
    commands.spawn_bundle(Camera2dBundle::default());

    // Add walls
    let wall_color = Color::rgb(0.85, 0.85, 0.85);
    let wall_thickness = 8.0;
    let bounds = Vec2::new(240.0, 240.0);
    // left
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        },
        ..Default::default()
    });
    // right
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        },
        ..Default::default()
    });
    // bottom
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
        sprite: Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        },
        ..Default::default()
    });
    // top
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
        sprite: Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        },
        ..Default::default()
    });

    // Add cell
    let cell = Cell::None;
    commands
        .spawn_bundle(SpriteBundle {
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
        })
        .insert(cell)
        .insert(CellTimer(Timer::from_seconds(2.0, true)));
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
            game.answer.reset();
            info!("reset answer");
        }
    }
}

/// Check answers.
fn score_system(mut game: ResMut<NBack>, mut query: Query<&CellTimer>) {
    if let Ok(timer) = query.get_single_mut() {
        if timer.just_finished() {
            game.check_answer();
        }
    }
}

/// User interface.
fn debug_ui(mut egui_context: ResMut<EguiContext>, mut game: ResMut<NBack>) {
    egui::Window::new("debug")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(format!("n back: {}", game.cells.n_back()));
            ui.label(format!("correct: {}", game.score.correct()));
            ui.label(format!("wrong: {}", game.score.wrong()));
            ui.label(format!("F1 score: {}", game.score.f1_score()));

            if ui.button("Restart").clicked() {
                game.restart()
            }
        });
}
