use bevy::{prelude::*, render::pass::ClearColor, window::WindowMode};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_nback::{
    constant::SPACING,
    cue::{Cell, Pigment},
    nback::GameState,
};

struct GlobalState {
    game: GameState,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            game: Default::default(),
        }
    }
}

struct CellMaterials {
    one: Handle<ColorMaterial>,
    two: Handle<ColorMaterial>,
    three: Handle<ColorMaterial>,
    four: Handle<ColorMaterial>,
    five: Handle<ColorMaterial>,
    six: Handle<ColorMaterial>,
}

impl CellMaterials {
    fn from(&self, pigment: Pigment) -> Handle<ColorMaterial> {
        match pigment {
            Pigment::A => self.one.clone(),
            Pigment::B => self.two.clone(),
            Pigment::C => self.three.clone(),
            Pigment::D => self.four.clone(),
            Pigment::E => self.five.clone(),
            Pigment::None => self.six.clone(),
        }
    }
}

impl FromWorld for CellMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        CellMaterials {
            one: materials.add(Color::rgb(1.0, 0.56, 0.0).into()),
            two: materials.add(Color::rgb(0.60, 0.05, 1.0).into()),
            three: materials.add(Color::rgb(1.0, 0.0, 0.65).into()),
            four: materials.add(Color::rgb(0.12, 1.0, 0.14).into()),
            five: materials.add(Color::rgb(0.12, 0.80, 1.0).into()),
            six: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum SystemLabel {
    ScoreCheck,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "nback!".to_string(),
            width: 360.,
            height: 640.,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<CellMaterials>()
        .insert_resource(GlobalState::default())
        .insert_resource(ClearColor(Color::rgb(0.15, 0.15, 0.15)))
        .add_startup_system(setup.system())
        .add_system(timer_system.system())
        .add_system(score_system.system().label(SystemLabel::ScoreCheck))
        .add_system(cue_system.system().after(SystemLabel::ScoreCheck))
        .add_system(answer_system.system().after(SystemLabel::ScoreCheck))
        .add_system(debug_ui.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cell_materials: Res<CellMaterials>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    // Add the game's entities to our world

    // audio
    let music = asset_server.load("sounds/Cyberpunk Moonlight Sonata.mp3");
    audio.play(music);

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Add walls
    let wall_material = materials.add(Color::rgb(0.85, 0.85, 0.85).into());
    let wall_thickness = 8.0;
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
    let cell_material = cell_materials.one.clone();
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
    cell_materials: Res<CellMaterials>,
    mut board_query: Query<(&Cell, &mut Transform, &mut Handle<ColorMaterial>, &Timer)>,
) {
    if let Ok((_, mut transform, mut material, timer)) = board_query.single_mut() {
        if timer.just_finished() {
            let new_cell = globals.game.cells.gen();
            info!("cue: {:?}", new_cell);
            transform.translation = new_cell.translation();

            let new_pigment = globals.game.pigments.gen();
            *material = cell_materials.from(new_pigment);
        }
    }
}

fn answer_system(
    mut globals: ResMut<GlobalState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&Timer>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        globals.game.answer.w();
    }
    if keyboard_input.pressed(KeyCode::A) {
        globals.game.answer.a();
    }
    if keyboard_input.pressed(KeyCode::S) {
        globals.game.answer.s();
    }
    if keyboard_input.pressed(KeyCode::D) {
        globals.game.answer.d();
    }

    if let Ok(timer) = query.single_mut() {
        if timer.just_finished() {
            globals.game.answer.reset();
            info!("reset answer");
        }
    }
}

fn score_system(mut globals: ResMut<GlobalState>, mut query: Query<&Timer>) {
    if let Ok(timer) = query.single_mut() {
        if timer.just_finished() {
            globals.game.check_answer();
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
            ui.label(format!("n back: {}", globals.game.cells.n_back()));
            ui.label(format!("correct: {}", globals.game.score.correct()));
            ui.label(format!("wrong: {}", globals.game.score.wrong()));
            ui.label(format!("F1 score: {}", globals.game.score.f1_score()));

            if ui.button("Restart").clicked() {
                globals.game.restart()
            }
        });
}
