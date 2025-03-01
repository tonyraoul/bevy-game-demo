use bevy::prelude::*;
use bevy::time::Timer;
use rand::Rng;

pub const PLATFORM_HEIGHT: f32 = 5.0;

#[derive(Component, Default)]
pub struct Enemy {
    pub state: EnemyState,
    pub target_position: Option<Vec3>,
    pub state_timer: Timer,
    pub is_fallen: bool,
    pub respawn_timer: Timer,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum EnemyState {
    Patrol,
    Chase,
    Fallen,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            state: EnemyState::Patrol,
            target_position: None,
            state_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            is_fallen: false,
            respawn_timer: Timer::from_seconds(3.0, TimerMode::Once),
        }
    }

    pub fn get_random_platform_position() -> Vec3 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-8.0..8.0);
        let z = rng.gen_range(-8.0..8.0);
        Vec3::new(x, PLATFORM_HEIGHT + 2.0, z)
    }
} 