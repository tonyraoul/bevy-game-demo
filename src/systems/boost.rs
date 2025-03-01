use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::components::{EnergyBoost, Player, Enemy, BoostIndicator};

const BOOST_FORCE: f32 = 15.0;
const BOOST_THRESHOLD: f32 = 0.95;

pub fn handle_boost(
    mut query: Query<(&mut EnergyBoost, &mut Velocity)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut boost, mut velocity) in query.iter_mut() {
        // Handle cooldown
        if boost.cooldown_timer.tick(time.delta()).finished() {
            boost.is_boosting = false;
        }

        // Handle recharge
        if !boost.is_boosting && boost.recharge_timer.tick(time.delta()).finished() {
            boost.energy = (boost.energy + time.delta_seconds() * 0.5).min(1.0);
        }

        // Apply boost for player
        if keyboard.just_pressed(KeyCode::Space) && boost.energy > 0.1 && !boost.is_boosting {
            apply_boost(&mut boost, &mut velocity);
        }
    }
}

pub fn handle_ai_boost(
    mut query: Query<(&mut EnergyBoost, &mut Velocity), With<Enemy>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    
    for (mut boost, mut velocity) in query.iter_mut() {
        // Handle cooldown and recharge same as player
        if boost.cooldown_timer.tick(time.delta()).finished() {
            boost.is_boosting = false;
        }

        if !boost.is_boosting && boost.recharge_timer.tick(time.delta()).finished() {
            boost.energy = (boost.energy + time.delta_seconds() * 0.5).min(1.0);
        }

        // Random chance to boost if energy is high enough
        if boost.energy > BOOST_THRESHOLD && !boost.is_boosting && rng.gen_bool(0.1) {
            apply_boost(&mut boost, &mut velocity);
        }
    }
}

fn apply_boost(boost: &mut EnergyBoost, velocity: &mut Velocity) {
    let boost_direction = velocity.linvel.normalize_or_zero();
    velocity.linvel += boost_direction * BOOST_FORCE;
    boost.energy = 0.0;
    boost.is_boosting = true;
    boost.cooldown_timer.reset();
    boost.recharge_timer.reset();
}

pub fn update_boost_indicator(
    query: Query<(&EnergyBoost, &Player)>,
    mut indicator_query: Query<&mut Style, With<BoostIndicator>>,
) {
    if let Ok((boost, _)) = query.get_single() {
        if let Ok(mut style) = indicator_query.get_single_mut() {
            style.width = Val::Percent(boost.energy * 100.0);
        }
    }
} 