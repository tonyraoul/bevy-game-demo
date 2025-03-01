use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::components::{EnergyBoost, Player, Enemy, BoostIndicator, BearScore};

const BOOST_FORCE: f32 = 15.0;
const BOOST_THRESHOLD: f32 = 0.95;
const BOOST_DURATION: f32 = 3.0;  // Increased duration
const BOOST_COOLDOWN: f32 = 2.0;  // Reduced cooldown
const ENERGY_CONSUMPTION_RATE: f32 = 0.4;  // Energy consumed per second while boosting

pub fn handle_boost(
    mut query: Query<(&mut EnergyBoost, &Player)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut boost, _) in query.iter_mut() {
        // Handle cooldown
        if boost.cooldown_timer.tick(time.delta()).finished() && boost.is_boosting {
            boost.is_boosting = false;
        }

        // Gradually consume energy while boosting
        if boost.is_boosting {
            // Consume energy over time
            boost.energy = (boost.energy - time.delta_seconds() * ENERGY_CONSUMPTION_RATE).max(0.0);
            
            // Stop boosting if energy is depleted
            if boost.energy <= 0.0 {
                boost.is_boosting = false;
                boost.cooldown_timer.reset();
                boost.recharge_timer.reset();
            }
        }
        // Handle recharge when not boosting
        else if boost.recharge_timer.tick(time.delta()).finished() {
            boost.energy = (boost.energy + time.delta_seconds() * 0.5).min(1.0);
        }

        // Start boosting when space is pressed
        if keyboard.just_pressed(KeyCode::Space) && boost.energy > 0.1 && !boost.is_boosting {
            apply_boost(&mut boost);
        }
        
        // Stop boosting when space is released
        if keyboard.just_released(KeyCode::Space) && boost.is_boosting {
            boost.is_boosting = false;
            boost.cooldown_timer.reset();
        }
    }
}

pub fn handle_ai_boost(
    mut query: Query<(&mut EnergyBoost, &Enemy)>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    
    for (mut boost, _) in query.iter_mut() {
        // Handle cooldown
        if boost.cooldown_timer.tick(time.delta()).finished() && boost.is_boosting {
            boost.is_boosting = false;
        }

        // Gradually consume energy while boosting
        if boost.is_boosting {
            // Consume energy over time
            boost.energy = (boost.energy - time.delta_seconds() * ENERGY_CONSUMPTION_RATE).max(0.0);
            
            // Stop boosting if energy is depleted
            if boost.energy <= 0.0 {
                boost.is_boosting = false;
                boost.cooldown_timer.reset();
                boost.recharge_timer.reset();
            }
        }
        // Handle recharge when not boosting
        else if boost.recharge_timer.tick(time.delta()).finished() {
            boost.energy = (boost.energy + time.delta_seconds() * 0.5).min(1.0);
        }

        // Random chance to boost if energy is high enough
        if boost.energy > BOOST_THRESHOLD && !boost.is_boosting && rng.gen_bool(0.1) {
            apply_boost(&mut boost);
        }
    }
}

fn apply_boost(boost: &mut EnergyBoost) {
    // Just set the boosting flag, energy will be consumed gradually
    boost.is_boosting = true;
    boost.cooldown_timer.reset();
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
