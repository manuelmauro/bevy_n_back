use bevy::{prelude::*, render::pass::ClearColor};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_nback::{constant::SPACING, cue::Cell, nback::GameState};

struct GlobalState {
    answer: bool,
    game: GameState,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            answer: false,
            game: Default::default(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum SystemLabel {
    ScoreCheck,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(GlobalState::default())
        .insert_resource(ClearColor(Color::rgb(0.15, 0.15, 0.15)))
        .add_startup_system(setup.system())
        .add_system(timer_system.system())
        .add_system(score_system.system().label(SystemLabel::ScoreCheck))
        .add_system(cue_system.system().after(SystemLabel::ScoreCheck))
        .add_system(answer_system.system())
        .add_system(debug_ui.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Add walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(240.0, 240.0);

    // left
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    });
    // right
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    });
    // bottom
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    });
    // top
    commands.spawn_bundle(SpriteBundle {
        material: wall_material,
        transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    });

    // Add cell
    let cell = Cell::None;
    let cell_material = materials.add(Color::rgb(0.66, 0.76, 0.0).into());
    commands
        .spawn_bundle(SpriteBundle {
            material: cell_material,
            sprite: Sprite::new(Vec2::new(
                (bounds.x - SPACING) / 3.0,
                (bounds.x - SPACING) / 3.0,
            )),
            transform: Transform::from_translation(cell.translation()),
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
    mut globals: ResMut<GlobalState>,
    mut board_query: Query<(&Cell, &mut Transform, &Timer)>,
) {
    if let Ok((_, mut transform, timer)) = board_query.single_mut() {
        if timer.just_finished() {
            let new_cue = globals.game.cues.gen();

            info!("cue: {:?}", new_cue);
            transform.translation = new_cue.translation();
        }
    }
}

fn answer_system(mut globals: ResMut<GlobalState>, keyboard_input: Res<Input<KeyCode>>) {
    if !globals.answer {
        if keyboard_input.pressed(KeyCode::Left) {
            info!("matching!");
            globals.answer = true;
        }
    }
}

fn score_system(mut globals: ResMut<GlobalState>, mut query: Query<&Timer>) {
    if let Ok(timer) = query.single_mut() {
        if timer.just_finished() {
            if globals.answer {
                if globals.game.cues.is_match() {
                    globals.game.score.record_tp();
                    info!("true_positive");
                } else {
                    globals.game.score.record_fp();
                    info!("false_positive");
                }
            } else {
                if globals.game.cues.is_match() {
                    globals.game.score.record_fn();
                    info!("false_neg");
                } else {
                    globals.game.score.record_tn();
                    info!("true_neg");
                }
            }

            globals.answer = false;
        }
    }
}

// Note the usage of `ResMut`. Even though `ctx` method doesn't require
// mutability, accessing the context from different threads will result
// into panic if you don't enable `egui/multi_threaded` feature.
fn debug_ui(egui_context: ResMut<EguiContext>, mut globals: ResMut<GlobalState>) {
    egui::Window::new("debug")
        .resizable(false)
        .show(egui_context.ctx(), |ui| {
            ui.label(format!("n back: {}", globals.game.cues.n_back()));
            ui.label(format!("correct: {}", globals.game.score.correct()));
            ui.label(format!("wrong: {}", globals.game.score.wrong()));
            ui.label(format!("F1 score: {}", globals.game.score.f1_score()));

            if ui.button("Restart").clicked() {
                globals.game.restart()
            }
        });
}
