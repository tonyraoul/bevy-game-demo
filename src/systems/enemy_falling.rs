use bevy::prelude::*;
use bevy_rapier3d::prelude::{Velocity};

use crate::components::{Enemy, EnemyState, PLATFORM_HEIGHT, BearScore};

const FALL_THRESHOLD: f32 = -5.0;
const RESPAWN_POSITION: Vec3 = Vec3::new(0.0, PLATFORM_HEIGHT + 2.0, 0.0);
const FALL_ACCELERATION: f32 = 30.0;  // Additional downward force when falling

pub fn handle_enemy_falls(
    mut enemy_query: Query<(&mut Enemy, &mut Transform, &mut Velocity, &mut BearScore)>,
    time: Res<Time>,
) {
    for (mut enemy, mut transform, mut velocity, mut score) in enemy_query.iter_mut() {
        // Apply extra downward force when falling
        if transform.translation.y < PLATFORM_HEIGHT && !enemy.is_fallen {
            velocity.linvel.y -= FALL_ACCELERATION * time.delta_seconds();
        }

        // Check if enemy has fallen
        if transform.translation.y < FALL_THRESHOLD && !enemy.is_fallen {
            // Enemy has fallen
            enemy.is_fallen = true;
            enemy.state = EnemyState::Fallen;
            enemy.respawn_timer.reset();
            score.value -= 1;  // Deduct a point
            
            // Keep the enemy at the bottom of the platform
            transform.translation.y = FALL_THRESHOLD;
            velocity.linvel = Vec3::ZERO;
            velocity.angvel = Vec3::ZERO;
        }

        // Handle respawn timer for fallen enemies
        if enemy.is_fallen {
            if enemy.respawn_timer.tick(time.delta()).finished() {
                // Respawn the enemy
                enemy.is_fallen = false;
                enemy.state = EnemyState::Patrol;
                transform.translation = RESPAWN_POSITION;
                velocity.linvel = Vec3::ZERO;
                velocity.angvel = Vec3::ZERO;
            }
        }
    }
}
