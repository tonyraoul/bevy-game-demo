use bevy::prelude::*;

#[derive(Component)]
pub struct EnergyBoost {
    pub energy: f32,
    pub is_boosting: bool,
    pub cooldown_timer: Timer,
    pub recharge_timer: Timer,
}

impl Default for EnergyBoost {
    fn default() -> Self {
        Self {
            energy: 1.0,
            is_boosting: false,
            cooldown_timer: Timer::from_seconds(1.0, TimerMode::Once),
            recharge_timer: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }
} 