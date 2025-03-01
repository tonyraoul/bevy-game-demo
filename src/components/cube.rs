use bevy::prelude::*;

#[derive(Component)]
pub struct SpinningCube {
    pub rotation_speed_x: f32,
    pub rotation_speed_y: f32,
}

impl Default for SpinningCube {
    fn default() -> Self {
        Self {
            rotation_speed_x: 0.5,
            rotation_speed_y: 1.0,
        }
    }
} 