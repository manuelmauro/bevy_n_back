use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use bevy_n_back::{
    game::GamePlugin,
    menu::MenuPlugin,
    setting::{DisplayQuality, Volume},
    splash::SplashPlugin,
    GameState,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((SplashPlugin, MenuPlugin, GamePlugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings {
            intensity: 0.5,
            low_frequency_boost: 1.0,
            high_pass_frequency: 1.0,
            low_frequency_boost_curvature: 1.0,
            ..default()
        },
    ));
}
