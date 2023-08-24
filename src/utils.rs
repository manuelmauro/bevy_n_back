use crate::game::core::cue::{Cell, Pigment};
use bevy::prelude::*;

pub const SIZE: f32 = 60.0;
pub const SPACING: f32 = 20.0;

impl From<&Cell> for Vec3 {
    fn from(cell: &Cell) -> Self {
        Vec3::new(
            column(cell) * (SIZE + SPACING),
            row(cell) * (SIZE + SPACING),
            0.0,
        )
    }
}

fn row(cell: &Cell) -> f32 {
    match &cell {
        Cell::TopLeft => 1.0,
        Cell::TopCenter => 1.0,
        Cell::TopRight => 1.0,
        Cell::CenterLeft => 0.0,
        Cell::Center => 0.0,
        Cell::CenterRight => 0.0,
        Cell::BottomLeft => -1.0,
        Cell::BottomCenter => -1.0,
        Cell::BottomRight => -1.0,
        Cell::None => 0.0,
    }
}

fn column(cell: &Cell) -> f32 {
    match &cell {
        Cell::TopLeft => -1.0,
        Cell::TopCenter => 0.0,
        Cell::TopRight => 1.0,
        Cell::CenterLeft => -1.0,
        Cell::Center => 0.0,
        Cell::CenterRight => 1.0,
        Cell::BottomLeft => -1.0,
        Cell::BottomCenter => 0.0,
        Cell::BottomRight => 1.0,
        Cell::None => 0.0,
    }
}

impl From<&Pigment> for Color {
    fn from(pigment: &Pigment) -> Self {
        match pigment {
            Pigment::A => Color::rgb(1.0, 0.56, 0.0),
            Pigment::B => Color::rgb(0.60, 0.05, 1.0),
            Pigment::C => Color::rgb(1.0, 0.0, 0.65),
            Pigment::D => Color::rgb(0.12, 1.0, 0.14),
            Pigment::E => Color::rgb(0.12, 0.80, 1.0),
            Pigment::None => Color::rgb(0.0, 0.0, 0.0),
        }
    }
}

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
