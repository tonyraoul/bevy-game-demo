use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub base_scale: Vec3,
    pub current_scale: Vec3,
}

impl Player {
    pub fn new(speed: f32) -> Self {
        let base_scale = Vec3::new(1.0, 1.0, 1.0);
        Self {
            speed,
            base_scale,
            current_scale: base_scale,
        }
    }
}
