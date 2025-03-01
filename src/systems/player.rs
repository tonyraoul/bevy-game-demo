use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::{Player, EnergyBoost};

const FALL_THRESHOLD: f32 = -10.0; // Y position that determines when a player has fallen
const SPAWN_POSITION: Vec3 = Vec3::new(0.0, 2.0, 0.0);

#[derive(Resource)]
pub struct Score {
    pub value: i32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 10 }
    }
}

pub fn player_movement(
    mut player_query: Query<(&Player, &mut Transform, &mut Velocity)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (player, mut transform, mut velocity) in player_query.iter_mut() {
        let mut movement = Vec3::ZERO;

        // Get keyboard input
        if keyboard.pressed(KeyCode::W) {
            movement.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::S) {
            movement.z += 1.0;
        }
        if keyboard.pressed(KeyCode::A) {
            movement.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::D) {
            movement.x += 1.0;
        }

        // Normalize movement vector to prevent faster diagonal movement
        if movement != Vec3::ZERO {
            movement = movement.normalize();
            
            // Calculate target rotation
            let target_rotation = if movement != Vec3::ZERO {
                Quat::from_rotation_y(-movement.z.atan2(movement.x))
            } else {
                transform.rotation
            };

            // Smoothly rotate towards movement direction
            transform.rotation = transform.rotation.slerp(
                target_rotation,
                time.delta_seconds() * player.rotation_speed
            );
        }

        // Apply movement
        velocity.linvel = movement * player.movement_speed;
    }
}

pub fn check_fall(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut score: ResMut<Score>,
) {
    for (mut transform, mut velocity) in player_query.iter_mut() {
        if transform.translation.y < FALL_THRESHOLD {
            // Player has fallen
            score.value -= 1;
            
            // Reset position and velocity
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
    // Temporary bear mesh (cube for now)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(1.0, 1.0, 1.5).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.6, 0.4, 0.2),
                ..default()
            }),
            transform: Transform::from_xyz(SPAWN_POSITION.x, SPAWN_POSITION.y, SPAWN_POSITION.z),
            ..default()
        },
        Player::default(),
        EnergyBoost::default(),
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::cuboid(0.5, 0.5, 0.75),
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        Damping {
            linear_damping: 5.0,
            angular_damping: 5.0,
        },
    ));
} 