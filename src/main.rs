mod components;
mod systems;
mod plugins;
mod styles;

use bevy::{
    prelude::*,
    window::WindowMode,
};
use plugins::{CubeDemoPlugin, MenuPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Demo - Spinning Cube".into(),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((CubeDemoPlugin, MenuPlugin))
        .run();
} 