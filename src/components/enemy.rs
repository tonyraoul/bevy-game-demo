use bevy::prelude::*;
use bevy::time::Timer;
use rand::Rng;

pub const PLATFORM_HEIGHT: f32 = 5.0;
pub const WEAK_THRESHOLD: f32 = 30.0;

#[derive(Component, Default)]
pub struct Enemy {
    pub state: EnemyState,
    pub target_position: Option<Vec3>,
    pub state_timer: Timer,
    pub is_fallen: bool,
    pub respawn_timer: Timer,
    pub health: f32,
    pub target: Option<Entity>,
    pub target_timer: Timer, // Add target_timer field
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub enum EnemyState {
    #[default]
    Patrol,
    Chase,
    Fallen,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            state: EnemyState::Patrol,
            target_position: None,
            state_timer: Timer::from_seconds(1.5, TimerMode::Repeating), // Faster state transitions
            is_fallen: false,
            respawn_timer: Timer::from_seconds(2.0, TimerMode::Once), // Faster respawn
            health: 100.0,
            target: None,
            target_timer: Timer::from_seconds(1.0, TimerMode::Repeating), // More frequent targeting // Initialize with a default value
        }
    }

    pub fn get_random_platform_position() -> Vec3 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-8.0..8.0);
        let z = rng.gen_range(-8.0..8.0);
        Vec3::new(x, PLATFORM_HEIGHT + 2.0, z)
    }

    pub fn is_weak(&self) -> bool {
        self.health <= WEAK_THRESHOLD
    }
}