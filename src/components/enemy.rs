use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Enemy {
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub state: EnemyState,
    pub target_position: Option<Vec3>,
    pub state_timer: Timer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyState {
    Patrol,
    Chase,
    Fight,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            movement_speed: 4.0,
            rotation_speed: 4.0,
            state: EnemyState::Patrol,
            target_position: None,
            state_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        }
    }
}

impl Enemy {
    pub fn get_random_platform_position() -> Vec3 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-8.0..8.0);
        let z = rng.gen_range(-8.0..8.0);
        Vec3::new(x, 2.0, z)
    }
} 