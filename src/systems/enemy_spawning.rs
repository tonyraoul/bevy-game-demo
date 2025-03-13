use bevy::prelude::*;
use bevy_rapier3d::prelude::{RigidBody, Velocity, Collider, LockedAxes, Damping, CollisionGroups, Group};

use crate::components::{Enemy, EnergyBoost, PLATFORM_HEIGHT, BearScore};

const SPAWN_POSITIONS: [(f32, f32); 4] = [
    (-8.0, -8.0),
    (-8.0, 8.0),
    (8.0, -8.0),
    (8.0, 8.0),
];

pub fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn initial enemies at corners
    for (i, (x, z)) in SPAWN_POSITIONS.iter().enumerate() {
        spawn_enemy(&mut commands, &mut meshes, &mut materials, Vec3::new(*x, PLATFORM_HEIGHT + 2.0, *z), format!("Enemy {}", i + 1));
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    name: String,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(1.0, 1.0, 1.5).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.2, 0.2),
                ..default()
            }),
            transform: Transform::from_translation(position),
            ..default()
        },
        Enemy::new(),
        BearScore::new(name),
        EnergyBoost::default(),
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::cuboid(0.5, 0.5, 0.75),
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        Damping {
            linear_damping: 0.1,  // Further reduced from 0.5
            angular_damping: 0.5,  // Further reduced from 1.0
        },
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_2),
    ));
}
