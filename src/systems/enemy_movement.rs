use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::components::{Enemy, EnemyState, Player, EnergyBoost};

// Physics constants
const BASE_MOVEMENT_FORCE: f32 = 18.0;
const MAX_SPEED: f32 = 7.0;
const FRICTION: f32 = 0.95;
const CHASE_DISTANCE: f32 = 8.0;

pub fn enemy_behavior(
    mut enemy_query: Query<(&mut Enemy, &Transform, &mut Velocity, &EnergyBoost)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    let player_pos = if let Ok(player_transform) = player_query.get_single() {
        player_transform.translation
    } else {
        return;
    };

    for (mut enemy, transform, mut velocity, boost) in enemy_query.iter_mut() {
        // Skip behavior for fallen enemies
        if enemy.is_fallen {
            continue;
        }

        // Update state timer
        enemy.state_timer.tick(time.delta());

        // More sophisticated state transitions
        if enemy.state_timer.just_finished() {
            let distance_to_player = transform.translation.distance(player_pos);
            
            // Weighted state transitions
            enemy.state = match enemy.state {
                EnemyState::Patrol => {
                    // Higher chance to chase if player is close
                    if distance_to_player < CHASE_DISTANCE && rng.gen_bool(0.7) {
                        EnemyState::Chase
                    } else if rng.gen_bool(0.3) {
                        // Occasional wandering or brief pause
                        EnemyState::Patrol
                    } else {
                        enemy.target_position = Some(Enemy::get_random_platform_position());
                        EnemyState::Patrol
                    }
                },
                EnemyState::Chase => {
                    // Return to patrol if player is far or randomly decide to stop chasing
                    if distance_to_player > CHASE_DISTANCE * 1.5 || rng.gen_bool(0.4) {
                        enemy.target_position = Some(Enemy::get_random_platform_position());
                        EnemyState::Patrol
                    } else {
                        EnemyState::Chase
                    }
                },
                EnemyState::Fallen => EnemyState::Patrol,
            };
        }

        // Handle behavior based on state with more nuanced movement
        let target_pos = match enemy.state {
            EnemyState::Patrol => {
                if enemy.target_position.is_none() {
                    enemy.target_position = Some(Enemy::get_random_platform_position());
                }
                enemy.target_position.unwrap()
            }
            EnemyState::Chase => {
                // Predictive chasing: aim slightly ahead of player
                let prediction_factor = 1.5;
                player_pos + (player_pos - transform.translation).normalize() * prediction_factor
            }
            EnemyState::Fallen => continue,
        };

        // Calculate movement direction with slight randomness
        let base_direction = (target_pos - transform.translation).normalize();
        let jitter_factor = match enemy.state {
            EnemyState::Patrol => 0.2,  // More wandering in patrol
            EnemyState::Chase => 0.1,   // More precise in chase
            _ => 0.0,
        };
        
        let jittered_direction = base_direction + Vec3::new(
            rng.gen_range(-jitter_factor..jitter_factor),
            0.0,
            rng.gen_range(-jitter_factor..jitter_factor)
        ).normalize();

        // Only move if not too close to target
        if transform.translation.distance(target_pos) > 2.0 {
            // Dynamic force based on state and boost
            let force_multiplier = match (enemy.state, boost.is_boosting) {
                (EnemyState::Chase, true) => 3.0,   // Aggressive chase with boost
                (EnemyState::Chase, false) => 2.0,  // Determined chase
                (EnemyState::Patrol, true) => 2.5,  // Boosted patrol
                _ => 1.0,
            };
            
            let force = BASE_MOVEMENT_FORCE * force_multiplier;
            velocity.linvel += jittered_direction * force * time.delta_seconds();

            // Adaptive friction and speed
            velocity.linvel *= match enemy.state {
                EnemyState::Chase => 0.98,  // Less friction when chasing
                _ => FRICTION,
            };

            // Dynamic max speed
            let max_speed = match (enemy.state, boost.is_boosting) {
                (EnemyState::Chase, true) => MAX_SPEED * 2.0,
                (EnemyState::Chase, false) => MAX_SPEED * 1.5,
                (EnemyState::Patrol, true) => MAX_SPEED * 1.3,
                _ => MAX_SPEED,
            };
            
            let speed = velocity.linvel.length();
            if speed > max_speed {
                velocity.linvel = velocity.linvel.normalize() * max_speed;
            }
        } else {
            // Gradual deceleration when near target
            velocity.linvel *= 0.9;
        }
    }
}
