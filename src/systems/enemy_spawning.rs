use bevy::prelude::*;
use bevy_rapier3d::prelude::{RigidBody, Velocity, Collider, LockedAxes, Damping, CollisionGroups, Group};
use rand::Rng;

use crate::components::{Enemy, EnemyState, EnergyBoost, PLATFORM_HEIGHT, BearScore, DuckParams, spawn_duck};

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
    let mut rng = rand::thread_rng();
    // Spawn initial enemies at corners
    for (i, (x, z)) in SPAWN_POSITIONS.iter().enumerate() {
        let health = rng.gen_range(50.0..100.0); // Random health between 50 and 100
        let mut enemy = Enemy::new();
        enemy.health = health;
        enemy.state = EnemyState::Chase;

        let enemy_entity = spawn_duck(
            &mut commands,
            &mut meshes,
            &mut materials,
            DuckParams {
                body_radius: 0.5,
                head_radius: 0.4,
                bill_length: 0.4,
                body_offset: Vec3::new(0.0, 0.0, 0.0),
                head_offset: Vec3::new(0.0, 0.7, 0.0),
                bill_offset: Vec3::new(0.2, 0.0, 0.0),
                base_color: Color::rgb(0.8, 0.2, 0.2),
                bill_color: Color::rgb(0.8, 0.6, 0.0),
                position: Vec3::new(*x, PLATFORM_HEIGHT + 2.0, *z),
                is_player: false,
            },
        );

        commands.entity(enemy_entity).insert((
            enemy,
            BearScore::new(format!("Enemy {}", i + 1)),
            EnergyBoost::default(),
        ));
    }
}
