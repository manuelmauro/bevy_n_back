use std::collections::VecDeque;

use bevy::{core::FixedTimestep, prelude::*, render::pass::ClearColor};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/// An implementation of the classic game "Dual-N-Back"
const TIME_STEP: f32 = 1.0;
const SIZE: f32 = 60.0;
const SPACING: f32 = 20.0;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard::default())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(position_system.system())
                .with_system(color_system.system())
                .with_system(history_system.system()),
        )
        .add_system(answer_system.system())
        .add_system(scoreboard_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

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
            Vertex::TopLeft => -1.0,
            Vertex::TopCenter => -1.0,
            Vertex::TopRight => -1.0,
            Vertex::CenterLeft => 0.0,
            Vertex::Center => 0.0,
            Vertex::CenterRight => 0.0,
            Vertex::BottomLeft => 1.0,
            Vertex::BottomCenter => 1.0,
            Vertex::BottomRight => 1.0,
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
}

impl Tint {
    fn material(&self) -> ColorMaterial {
        match self {
            Tint::Red => Color::rgb(1.0, 0.0, 0.0).into(),
            Tint::Green => Color::rgb(0.0, 1.0, 0.0).into(),
            Tint::Blue => Color::rgb(0.0, 0.0, 1.0).into(),
        }
    }
    fn next(&self) -> Self {
        rand::random()
    }
}

impl Distribution<Tint> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tint {
        match rng.gen_range(0..=9) {
            0 => Tint::Red,
            1 => Tint::Green,
            _ => Tint::Blue,
        }
    }
}

struct Paint {
    color: Tint,
}

struct Scoreboard {
    score: usize,
    position_history: VecDeque<Vertex>,
    color_history: VecDeque<Tint>,
}

impl Default for Scoreboard {
    fn default() -> Self {
        let mut position_history = VecDeque::new();
        position_history.push_front(Vertex::Center);
        position_history.push_front(Vertex::Center);
        position_history.push_front(Vertex::Center);

        let mut color_history = VecDeque::new();
        color_history.push_front(Tint::Blue);
        color_history.push_front(Tint::Blue);
        color_history.push_front(Tint::Blue);

        Scoreboard {
            score: 0,
            position_history,
            color_history,
        }
    }
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
        vertex: Vertex::Center,
    };
    let cell_paint = Paint { color: Tint::Red };
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
        .insert(cell_paint);
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = format!("Score: {}", scoreboard.score);
}

fn history_system(mut scoreboard: ResMut<Scoreboard>, mut board_query: Query<(&Position, &Paint)>) {
    if let Ok((position, paint)) = board_query.single_mut() {
        scoreboard.position_history.pop_front();
        scoreboard
            .position_history
            .push_back(position.vertex.clone());
        scoreboard.color_history.pop_front();
        scoreboard.color_history.push_back(paint.color.clone());
    }
}

fn position_system(mut board_query: Query<(&mut Position, &mut Transform)>) {
    if let Ok((mut position, mut transform)) = board_query.single_mut() {
        position.vertex = position.vertex.next();
        info!("position: {:?}", position.vertex);
        transform.translation = position.vertex.translation();
    }
}

fn color_system(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut board_query: Query<(&mut Paint, &mut Handle<ColorMaterial>)>,
) {
    if let Ok((mut paint, mut material)) = board_query.single_mut() {
        paint.color = paint.color.next();
        info!("color: {:?}", paint.color);
        let new_material = materials.add(paint.color.material());
        *material = new_material;
    }
}

fn answer_system(
    mut scoreboard: ResMut<Scoreboard>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Position, &Paint)>,
) {
    if let Ok((position, paint)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            if position.vertex == *scoreboard.position_history.front().unwrap() {
                scoreboard.score += 1;
            }
        }

        if keyboard_input.pressed(KeyCode::Right) {
            if paint.color == *scoreboard.color_history.front().unwrap() {
                scoreboard.score += 1;
            }
        }
    }
}
