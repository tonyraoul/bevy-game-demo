use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::{Player, EnergyBoost, PLATFORM_HEIGHT};
use crate::systems::Score;

const FALL_THRESHOLD: f32 = -5.0;
const SPAWN_POSITION: Vec3 = Vec3::new(0.0, PLATFORM_HEIGHT + 2.0, 0.0);

// Physics constants
const BASE_MOVEMENT_FORCE: f32 = 20.0;
const MAX_SPEED: f32 = 8.0;
const FRICTION: f32 = 0.95;
const PUSH_FORCE: f32 = 10.0;

pub fn player_movement(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (_transform, mut velocity) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;

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

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            velocity.linvel += direction * BASE_MOVEMENT_FORCE * time.delta_seconds();
        }

        // Apply friction
        velocity.linvel *= FRICTION;

        // Clamp maximum speed
        let speed = velocity.linvel.length();
        if speed > MAX_SPEED {
            velocity.linvel = velocity.linvel.normalize() * MAX_SPEED;
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
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut score: ResMut<Score>,
) {
    for (mut transform, mut velocity) in player_query.iter_mut() {
        if transform.translation.y < FALL_THRESHOLD {
            // Reset player position
            transform.translation = SPAWN_POSITION;
            velocity.linvel = Vec3::ZERO;
            velocity.angvel = Vec3::ZERO;
            
            // Deduct points
            score.value -= 2;
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(1.0, 1.0, 1.5).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 0.7, 0.2),
                ..default()
            }),
            transform: Transform::from_translation(SPAWN_POSITION),
            ..default()
        },
        Player,
        EnergyBoost::default(),
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::cuboid(0.5, 0.5, 0.75),
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        Damping {
            linear_damping: 5.0,
            angular_damping: 5.0,
        },
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2),  // Player can collide with environment and other bears
    ));
} 