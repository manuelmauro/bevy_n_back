use bevy::prelude::*;

pub mod game;
pub mod menu;
pub mod setting;
pub mod splash;
pub mod utils;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}
