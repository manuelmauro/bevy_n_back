use bevy::prelude::Component;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone, Component, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum Pigment {
    A,
    B,
    C,
    D,
    E,
    None,
}

impl Default for Pigment {
    fn default() -> Self {
        Pigment::None
    }
}

impl Distribution<Pigment> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Pigment {
        match rng.gen_range(0..=5) {
            0 => Pigment::A,
            1 => Pigment::B,
            2 => Pigment::C,
            3 => Pigment::D,
            _ => Pigment::E,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Letter {
    C,
    H,
    K,
    L,
    Q,
    R,
    S,
    T,
    None,
}

impl Default for Letter {
    fn default() -> Self {
        Letter::None
    }
}

impl Distribution<Letter> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Letter {
        match rng.gen_range(0..=7) {
            0 => Letter::C,
            1 => Letter::H,
            2 => Letter::K,
            3 => Letter::L,
            4 => Letter::Q,
            5 => Letter::R,
            6 => Letter::S,
            _ => Letter::T,
        }
    }
}
