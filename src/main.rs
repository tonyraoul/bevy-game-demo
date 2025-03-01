mod components;
mod systems;
mod plugins;
mod styles;
mod states;

use bevy::{
    prelude::*,
    window::WindowMode,
};
use plugins::{MenuPlugin, GamePlugin, SettingsPlugin};
use states::GameState;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Demo - Spinning Cube".into(),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((MenuPlugin, GamePlugin, SettingsPlugin))
        .run();
} 