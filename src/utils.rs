use crate::constant::{SIZE, SPACING};
use crate::nback::cue::Cell;
use bevy::prelude::*;

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
