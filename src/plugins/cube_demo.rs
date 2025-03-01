use bevy::prelude::*;
use crate::systems::{setup_camera_and_light, spawn_cube, rotate_cube};

pub struct CubeDemoPlugin;

impl Plugin for CubeDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera_and_light, spawn_cube))
            .add_systems(Update, rotate_cube);
    }
} 