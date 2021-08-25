use super::constant::{SIZE, SPACING};
use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
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

impl Cell {
    pub fn translation(&self) -> Vec3 {
        Vec3::new(
            self.column() * (SIZE + SPACING),
            self.row() * (SIZE + SPACING),
            0.0,
        )
    }

    fn row(&self) -> f32 {
        match &self {
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

    fn column(&self) -> f32 {
        match &self {
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
}

impl Default for Cell {
    fn default() -> Self {
        Cell::None
    }
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        match rng.gen_range(0..=9) {
            0 => Cell::TopLeft,
            1 => Cell::TopCenter,
            2 => Cell::TopRight,
            3 => Cell::CenterLeft,
            4 => Cell::Center,
            5 => Cell::CenterRight,
            6 => Cell::BottomLeft,
            7 => Cell::BottomCenter,
            _ => Cell::BottomRight,
        }
    }
}
