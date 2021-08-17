use bevy::{prelude::*, render::pass::ClearColor};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::collections::VecDeque;

const SIZE: f32 = 60.0;
const SPACING: f32 = 20.0;

#[derive(Clone, Debug, PartialEq)]
enum Vertex {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    None,
}

impl Vertex {
    fn translation(&self) -> Vec3 {
        Vec3::new(
            &self.column() * (SIZE + SPACING),
            &self.row() * (SIZE + SPACING),
            0.0,
        )
    }

    fn row(&self) -> f32 {
        match &self {
            Vertex::TopLeft => 1.0,
            Vertex::TopCenter => 1.0,
            Vertex::TopRight => 1.0,
            Vertex::CenterLeft => 0.0,
            Vertex::Center => 0.0,
            Vertex::CenterRight => 0.0,
            Vertex::BottomLeft => -1.0,
            Vertex::BottomCenter => -1.0,
            Vertex::BottomRight => -1.0,
            Vertex::None => 0.0,
        }
    }

    fn column(&self) -> f32 {
        match &self {
            Vertex::TopLeft => -1.0,
            Vertex::TopCenter => 0.0,
            Vertex::TopRight => 1.0,
            Vertex::CenterLeft => -1.0,
            Vertex::Center => 0.0,
            Vertex::CenterRight => 1.0,
            Vertex::BottomLeft => -1.0,
            Vertex::BottomCenter => 0.0,
            Vertex::BottomRight => 1.0,
            Vertex::None => 0.0,
        }
    }

    fn next(&self) -> Self {
        rand::random()
    }
}

impl Distribution<Vertex> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vertex {
        match rng.gen_range(0..=9) {
            0 => Vertex::TopLeft,
            1 => Vertex::TopCenter,
            2 => Vertex::TopRight,
            3 => Vertex::CenterLeft,
            4 => Vertex::Center,
            5 => Vertex::CenterRight,
            6 => Vertex::BottomLeft,
            7 => Vertex::BottomCenter,
            _ => Vertex::BottomRight,
        }
    }
}

struct Position {
    vertex: Vertex,
}

#[derive(Clone, Debug, PartialEq)]
enum Tint {
    Red,
    Green,
    Blue,
    None,
}

impl Tint {
    fn material(&self) -> ColorMaterial {
        match self {
            Tint::Red => Color::rgb(1.0, 0.0, 0.0).into(),
            Tint::Green => Color::rgb(0.0, 1.0, 0.0).into(),
            Tint::Blue => Color::rgb(0.0, 0.0, 1.0).into(),
            Tint::None => Color::rgb(0.0, 0.0, 0.0).into(),
        }
    }
    fn next(&self) -> Self {
        rand::random()
    }
}

impl Distribution<Tint> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tint {
        match rng.gen_range(0..=2) {
            0 => Tint::Red,
            1 => Tint::Green,
            _ => Tint::Blue,
        }
    }
}

struct Paint {
    color: Tint,
}

struct Score {
    false_pos: usize,
    true_pos: usize,
    false_neg: usize,
    true_neg: usize,
}

impl Score {
    fn correct(&self) -> usize {
        self.true_pos + self.true_neg
    }

    fn wrong(&self) -> usize {
        self.false_pos + self.false_neg
    }

    fn f1_score(&self) -> f32 {
        self.true_pos as f32
            / (self.true_pos as f32 + 0.5 * (self.false_pos as f32 + self.false_neg as f32))
    }
}

impl Default for Score {
    fn default() -> Self {
        Score {
            false_pos: 0,
            true_pos: 0,
            false_neg: 0,
            true_neg: 0,
        }
    }
}

struct GlobalState {
    score_position: Score,
    score_color: Score,
    position_answer: bool,
    color_answer: bool,
    position_history: VecDeque<Vertex>,
    color_history: VecDeque<Tint>,
}

impl Default for GlobalState {
    fn default() -> Self {
        let mut position_history = VecDeque::new();
        position_history.push_front(Vertex::None);
        position_history.push_front(Vertex::None);

        let mut color_history = VecDeque::new();
        color_history.push_front(Tint::None);
        color_history.push_front(Tint::None);

        GlobalState {
            score_position: Default::default(),
            score_color: Default::default(),
            position_answer: false,
            color_answer: false,
            position_history,
            color_history,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum MyStage {
    BeforeRound,
    AfterRound,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalState::default())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_stage_before(
            CoreStage::Update,
            MyStage::BeforeRound,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::Update,
            MyStage::AfterRound,
            SystemStage::parallel(),
        )
        .add_system_to_stage(MyStage::BeforeRound, timer_system.system())
        .add_system_to_stage(MyStage::BeforeRound, score_system.system())
        .add_system_to_stage(CoreStage::Update, position_system.system())
        .add_system_to_stage(CoreStage::Update, color_system.system())
        .add_system_to_stage(CoreStage::Update, answer_system.system())
        .add_system_to_stage(MyStage::AfterRound, scoreboard_system.system())
        .add_system_to_stage(
            MyStage::AfterRound,
            bevy::input::system::exit_on_esc_system.system(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

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
    let cell_position = Position {
        vertex: Vertex::None,
    };
    let cell_paint = Paint { color: Tint::None };
    let cell_material = materials.add(cell_paint.color.material());
    commands
        .spawn_bundle(SpriteBundle {
            material: cell_material.clone(),
            sprite: Sprite::new(Vec2::new(
                (bounds.x - SPACING) / 3.0,
                (bounds.x - SPACING) / 3.0,
            )),
            transform: Transform::from_translation(cell_position.vertex.translation()),
            ..Default::default()
        })
        .insert(cell_position)
        .insert(cell_paint)
        .insert(Timer::from_seconds(2.0, true));
}

fn scoreboard_system(scoreboard: Res<GlobalState>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = format!(
        "Correct: {}, Wrong: {}, Score: {}",
        scoreboard.score_position.correct() + scoreboard.score_color.correct(),
        scoreboard.score_position.wrong() + scoreboard.score_color.wrong(),
        scoreboard.score_position.f1_score() * scoreboard.score_color.f1_score(),
    );
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

fn position_system(
    mut scoreboard: ResMut<GlobalState>,
    mut board_query: Query<(&mut Position, &mut Transform, &Timer)>,
) {
    if let Ok((mut position, mut transform, timer)) = board_query.single_mut() {
        if timer.just_finished() {
            scoreboard.position_history.pop_front();
            scoreboard
                .position_history
                .push_back(position.vertex.clone());

            position.vertex = position.vertex.next();
            info!("position: {:?}", position.vertex);
            transform.translation = position.vertex.translation();
        }
    }
}

fn color_system(
    mut scoreboard: ResMut<GlobalState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut board_query: Query<(&mut Paint, &mut Handle<ColorMaterial>, &Timer)>,
) {
    if let Ok((mut paint, mut material, timer)) = board_query.single_mut() {
        if timer.just_finished() {
            scoreboard.color_history.pop_front();
            scoreboard.color_history.push_back(paint.color.clone());

            paint.color = paint.color.next();
            info!("color: {:?}", paint.color);
            let new_material = materials.add(paint.color.material());
            *material = new_material;
        }
    }
}

fn answer_system(mut scoreboard: ResMut<GlobalState>, keyboard_input: Res<Input<KeyCode>>) {
    if !scoreboard.position_answer {
        if keyboard_input.pressed(KeyCode::Left) {
            info!("same position!");
            scoreboard.position_answer = true;
        }
    }

    if !scoreboard.color_answer {
        if keyboard_input.pressed(KeyCode::Right) {
            info!("same color!");
            scoreboard.color_answer = true;
        }
    }
}

fn score_system(
    mut scoreboard: ResMut<GlobalState>,
    mut query: Query<&Timer>,
) {
    if let Ok(timer) = query.single_mut() {
        if timer.just_finished() {
            info!("checking answer");
            if scoreboard.position_answer {
                if *scoreboard.position_history.back().unwrap()  == *scoreboard.position_history.front().unwrap() {
                    scoreboard.score_position.true_pos += 1;
                    info!("position: true_positive");
                } else {
                    scoreboard.score_position.false_pos += 1;
                    info!("position: false_positive");
                }
            } else {
                if *scoreboard.position_history.back().unwrap()  == *scoreboard.position_history.front().unwrap() {
                    scoreboard.score_position.false_neg += 1;
                    info!("position: false_neg");
                } else {
                    scoreboard.score_position.true_neg += 1;
                    info!("position: true_neg");
                }
            }

            if scoreboard.color_answer {
                if *scoreboard.color_history.back().unwrap() == *scoreboard.color_history.front().unwrap() {
                    scoreboard.score_color.true_pos += 1;
                    info!("color: true_positive");
                } else {
                    scoreboard.score_color.false_pos += 1;
                    info!("color: false_positive");
                }
            } else {
                if *scoreboard.color_history.back().unwrap() == *scoreboard.color_history.front().unwrap() {
                    scoreboard.score_color.false_neg += 1;
                    info!("color: false_neg");
                } else {
                    scoreboard.score_color.true_neg += 1;
                    info!("color: true_neg");
                }
            }

            scoreboard.position_answer = false;
            scoreboard.color_answer = false;
        }
    }
}
