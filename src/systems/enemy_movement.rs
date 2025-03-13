use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::components::{Enemy, EnemyState, Player, EnergyBoost};

// Physics constants
const BASE_MOVEMENT_FORCE: f32 = 18.0;
const MAX_SPEED: f32 = 7.0;
const FRICTION: f32 = 0.95;

pub fn enemy_behavior(
    mut enemy_query: Query<(Entity, (&mut Enemy, &Transform, &mut Velocity, &EnergyBoost))>,
    player_query: Query<(Entity, &Transform, &Velocity), (With<Player>, Without<Enemy>)>, // Include player velocity
    all_enemies_query: Query<(Entity, &Transform), With<Enemy>>, // Query for all enemies and their health, excluding EnergyBoost and current enemy
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    let player_entity = player_query.get_single().ok().map(|(entity, _, _)| entity);
    let (player_pos, player_vel) = if let Ok((_, player_transform, player_velocity)) =
        player_query.get_single()
    {
        (player_transform.translation, player_velocity.linvel)
    } else {
        return;
    };

    for (enemy_entity, (mut enemy, transform, mut velocity, boost)) in enemy_query.iter_mut() {
        if enemy.is_fallen {
            continue;
        }

        enemy.state_timer.tick(time.delta());

        if enemy.state_timer.just_finished() {
            enemy.state = match enemy.state {
                EnemyState::Patrol => {
                    if rng.gen_bool(0.7) {
                        EnemyState::Chase
                    } else {
                        enemy.target_position = Some(Enemy::get_random_platform_position());
                        EnemyState::Patrol
                    }
                }
                EnemyState::Chase => {
                    if rng.gen_bool(0.4) {
                        enemy.target = None; // Clear target when switching to patrol
                        enemy.target_position = Some(Enemy::get_random_platform_position());
                        EnemyState::Patrol
                    } else {
                        EnemyState::Chase
                    }
                }
                EnemyState::Fallen => EnemyState::Patrol,
            };
        }

        // Simplified targeting logic
        if enemy.state == EnemyState::Chase {
            let mut target_pos = None;

            // If a target is already set, use it
            if let Some(target_entity) = enemy.target {
                if let Ok((_, target_transform)) = all_enemies_query.get(target_entity) {
                    target_pos = Some(target_transform.translation);
                } else if let Some(player_entity) = player_entity {
                    if enemy.target.as_ref() == Some(&player_entity) { // Corrected comparison
                        if let Ok((_, player_transform, player_velocity)) = player_query.get_single() {
                            target_pos = Some(player_transform.translation);
                        }
                    }
                } else {
                    enemy.target = None; // Clear invalid target
                }
            } else {
                // Try to target the player first
                if let Some(player_entity) = &player_entity {
                    enemy.target = Some(player_entity.clone()); // Correctly set target to player entity
                    if let Ok((_, player_transform, player_velocity)) = player_query.get_single() {
                        target_pos = Some(player_transform.translation);
                    }
                } else {
                    // If no player, target any enemy
                    for (entity, enemy_transform) in all_enemies_query.iter() { // Get entity and transform
                        if entity != enemy_entity {
                            enemy.target = Some(entity); // Correctly set target to enemy entity
                            target_pos = Some(enemy_transform.translation); // Use enemy_transform here
                            break;
                        }
                    }
                }
            }


            if let Some(target_pos) = target_pos {
                // Basic movement towards the target (no prediction or weakness check)
                let base_direction = (target_pos - transform.translation).normalize();
                let force = BASE_MOVEMENT_FORCE; // Use a basic force
                velocity.linvel += base_direction * force * time.delta_seconds();
                velocity.linvel *= FRICTION;
                let speed = velocity.linvel.length();
                if speed > MAX_SPEED {
                    velocity.linvel = velocity.linvel.normalize() * MAX_SPEED;
                }
            }
        } else if enemy.state == EnemyState::Patrol {
            if enemy.target_position.is_none() {
                enemy.target_position = Some(Enemy::get_random_platform_position());
            }
            if let Some(target_pos) = enemy.target_position {
                let base_direction = (target_pos - transform.translation).normalize();
                let force = BASE_MOVEMENT_FORCE;
                velocity.linvel += base_direction * force * time.delta_seconds();
                velocity.linvel *= FRICTION;
                let speed = velocity.linvel.length();
                if speed > MAX_SPEED {
                    velocity.linvel = velocity.linvel.normalize() * MAX_SPEED;
                }
            }
        }
    }
}
