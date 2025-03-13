use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::{Player, EnergyBoost, PLATFORM_HEIGHT, BearScore, CompoundSphereParams, spawn_compound_sphere};
use crate::states::GameState;

const FALL_THRESHOLD: f32 = -5.0;
const SPAWN_POSITION: Vec3 = Vec3::new(0.0, PLATFORM_HEIGHT + 2.0, 0.0);

// Physics constants
const BASE_MOVEMENT_FORCE: f32 = 25.0;
const MAX_SPEED: f32 = 8.0;
const BOOST_MAX_SPEED: f32 = 15.0;  // Higher max speed when boosting
/// Friction constant (currently unused but potentially useful for future physics tweaks)
#[allow(dead_code)]
const FRICTION: f32 = 0.9;

/// Push force constant (currently unused but potentially useful for future collision mechanics)
#[allow(dead_code)]
const PUSH_FORCE: f32 = 10.0;
const FALL_ACCELERATION: f32 = 30.0;  // Additional downward force when falling

pub fn player_movement(
    mut player_query: Query<(&mut Transform, &mut Velocity, &EnergyBoost), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (_transform, mut velocity, boost) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        // Diagonal movement handling with normalized speed
        if !boost.is_boosting {
            if keyboard_input.pressed(KeyCode::W) {
                direction.z -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::S) {
                direction.z += 1.0;
            }
            if keyboard_input.pressed(KeyCode::A) {
                direction.x -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::D) {
                direction.x += 1.0;
            }
        }

        if boost.is_boosting {
            // Always push forward when boosting (negative Z direction)
            direction = Vec3::new(0.0, 0.0, -1.0);
        }

        if direction != Vec3::ZERO {
            // Normalize diagonal movement
            direction = direction.normalize();
            
            // Smooth acceleration
            let acceleration = if boost.is_boosting {
                BASE_MOVEMENT_FORCE * 5.0  // Enhanced boost acceleration
            } else {
                BASE_MOVEMENT_FORCE * 1.2  // Smoother base acceleration
            };
            
            velocity.linvel += direction * acceleration * time.delta_seconds();
        } else {
            // Gradual deceleration when no input
            velocity.linvel *= 0.9;
        }

        // Clamp maximum speed, with higher limit when boosting
        let speed = velocity.linvel.length();
        let max_speed = if boost.is_boosting {
            BOOST_MAX_SPEED * 1.2  // Slightly higher boost speed
        } else {
            MAX_SPEED
        };
        
        if speed > max_speed {
            velocity.linvel = velocity.linvel.normalize() * max_speed;
        }
    }
}

pub fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(&Transform, &mut Velocity), With<Player>>,
    other_query: Query<&Transform, Without<Player>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            // Get the player and other entity's transform and velocity
            let (player_entity, other_entity) = if let Ok(_) = player_query.get_mut(*e1) {
                (*e1, *e2)
            } else if let Ok(_) = player_query.get_mut(*e2) {
                (*e2, *e1)
            } else {
                continue;
            };

            if let (Ok((player_transform, mut player_velocity)), Ok(other_transform)) = (
                player_query.get_mut(player_entity),
                other_query.get(other_entity),
            ) {
                // Calculate push direction
                let direction = (player_transform.translation - other_transform.translation).normalize();
                player_velocity.linvel += direction * PUSH_FORCE;
            }
        }
    }
}

pub fn check_fall(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut BearScore), With<Player>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (mut transform, mut velocity, mut score) in player_query.iter_mut() {
        // Apply extra downward force when falling
        if transform.translation.y < PLATFORM_HEIGHT {
            velocity.linvel.y -= FALL_ACCELERATION * time.delta_seconds();
        }

        if transform.translation.y < FALL_THRESHOLD {
            // Deduct points
            score.value -= 1;
            
            // Check if player has lost all points
            if score.value <= 0 {
                // Trigger game over
                next_state.set(GameState::GameOver);
                return;
            }
            
            // Reset player position
            transform.translation = SPAWN_POSITION;
            velocity.linvel = Vec3::ZERO;
            velocity.angvel = Vec3::ZERO;
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let params = CompoundSphereParams {
        base_radius: 0.5,
        head_radius: 0.3,
        ear_radius: 0.15,
        head_offset: Vec3::new(0.0, 0.6, 0.0),
        left_ear_offset: Vec3::new(-0.2, 0.2, 0.0),
        right_ear_offset: Vec3::new(0.2, 0.2, 0.0),
        base_color: Color::rgb(0.2, 0.7, 0.2),
        position: SPAWN_POSITION,
        is_player: true,
    };

    let entity = spawn_compound_sphere(
        &mut commands,
        &mut meshes,
        &mut materials,
        params,
    );

    commands.entity(entity).insert((
        Player::new(8.0),
        BearScore::new("Player".to_string()),
        EnergyBoost::default(),
        crate::components::ActivePowerUp::default(),
    ));
}
