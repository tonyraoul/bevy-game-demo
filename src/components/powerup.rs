use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum PowerUpType {
    Grow,
    Shrink,
}

#[derive(Component)]
pub struct PowerUp {
    /// Type of powerup used in powerup application logic
    pub power_type: PowerUpType,
    pub duration: Timer,
}

impl PowerUp {
    pub fn new(power_type: PowerUpType, duration: f32) -> Self {
        Self {
            power_type,
            duration: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

#[derive(Component, Default)]
pub struct ActivePowerUp {
    pub grow: Option<PowerUp>,
    pub shrink: Option<PowerUp>,
}
