// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, WindowDescriptor};
use bevy::DefaultPlugins;
use bevy::{prelude::*, window::WindowMode};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_kira_audio::{Audio, AudioPlugin};
use bevy_n_back::{
    constant::SPACING,
    nback::cue::{Cell, Pigment},
    nback::NBack,
};

struct CellMaterials {
    one: Color,
    two: Color,
    three: Color,
    four: Color,
    five: Color,
    six: Color,
}

impl CellMaterials {
    fn from(&self, pigment: Pigment) -> Color {
        match pigment {
            Pigment::A => self.one,
            Pigment::B => self.two,
            Pigment::C => self.three,
            Pigment::D => self.four,
            Pigment::E => self.five,
            Pigment::None => self.six,
        }
    }
}

impl FromWorld for CellMaterials {
    fn from_world(_world: &mut World) -> Self {
        CellMaterials {
            one: Color::rgb(1.0, 0.56, 0.0),
            two: Color::rgb(0.60, 0.05, 1.0),
            three: Color::rgb(1.0, 0.0, 0.65),
            four: Color::rgb(0.12, 1.0, 0.14),
            five: Color::rgb(0.12, 0.80, 1.0),
            six: Color::rgb(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum Label {
    ScoreCheck,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "nback!".to_string(),
        width: 360.,
        height: 640.,
        mode: WindowMode::Windowed,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_plugin(AudioPlugin)
    .init_resource::<CellMaterials>()
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

fn setup(
    mut commands: Commands,
    cell_materials: Res<CellMaterials>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    // Add the game's entities to our world

    // audio
    audio.play_looped(asset_server.load("sounds/Cyberpunk-Moonlight-Sonata.flac"));

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Add walls
    let wall_material = Color::rgb(0.85, 0.85, 0.85);
    let wall_thickness = 8.0;
    let bounds = Vec2::new(240.0, 240.0);

    // left
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite {
            color: wall_material,
            custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        },
        ..Default::default()
    });
    // right
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite {
            color: wall_material,
            custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        },
        ..Default::default()
    });
    // bottom
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
        sprite: Sprite {
            color: wall_material,
            custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        },
        ..Default::default()
    });
    // top
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
        sprite: Sprite {
            color: wall_material,
            custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        },
        ..Default::default()
    });

    // Add cell
    let cell = Cell::None;
    let cell_material = cell_materials.one;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: cell_material,
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
        .insert(Timer::from_seconds(2.0, true));
}

/// This system ticks all the `Timer` components on entities within the scene
/// using bevy's `Time` resource to get the delta between each update.
fn timer_system(time: Res<Time>, mut query: Query<&mut Timer>) {
    for mut timer in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            info!("tick!")
        }
    }
}

fn cue_system(
    mut game: ResMut<NBack>,
    cell_materials: Res<CellMaterials>,
    mut board_query: Query<(&Cell, &mut Transform, &mut Sprite, &Timer)>,
) {
    if let Ok((_, mut transform, mut sprite, timer)) = board_query.get_single_mut() {
        if timer.just_finished() {
            if let Some((new_cell, new_pigment)) = game.next() {
                info!("cue: {:?}", new_cell);
                transform.translation = (&new_cell).into();
                sprite.color = cell_materials.from(new_pigment);
            }
        }
    }
}

fn answer_system(
    mut game: ResMut<NBack>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&Timer>,
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

fn score_system(mut game: ResMut<NBack>, mut query: Query<&Timer>) {
    if let Ok(timer) = query.get_single_mut() {
        if timer.just_finished() {
            game.check_answer();
        }
    }
}

// Note the usage of `ResMut`. Even though `ctx` method doesn't require
// mutability, accessing the context from different threads will result
// into panic if you don't enable `egui/multi_threaded` feature.
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
