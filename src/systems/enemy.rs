use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::components::{Enemy, EnemyState, Player, EnergyBoost};

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
    for (x, z) in SPAWN_POSITIONS.iter() {
        spawn_enemy(&mut commands, &mut meshes, &mut materials, Vec3::new(*x, 2.0, *z));
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
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
        Enemy::default(),
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
        // Update state timer
        enemy.state_timer.tick(time.delta());

        if enemy.state_timer.just_finished() {
            // Randomly change state
            if rng.gen_bool(0.3) {
                enemy.state = match enemy.state {
                    EnemyState::Patrol => EnemyState::Chase,
                    EnemyState::Chase => EnemyState::Patrol,
                    EnemyState::Fight => EnemyState::Patrol,
                };
            }

            // Get new target position for patrol
            if enemy.state == EnemyState::Patrol {
                enemy.target_position = Some(Enemy::get_random_platform_position());
            }
        }

        // Handle behavior based on state
        let target_pos = match enemy.state {
            EnemyState::Patrol => {
                if enemy.target_position.is_none() {
                    enemy.target_position = Some(Enemy::get_random_platform_position());
                }
                enemy.target_position.unwrap()
            }
            EnemyState::Chase => player_pos,
            EnemyState::Fight => continue, // TODO: Implement fighting behavior
        };

        // Calculate movement direction
        let direction = (target_pos - transform.translation).normalize();
        
        // Only move if not too close to target
        if transform.translation.distance(target_pos) > 2.0 {
            // Apply movement with boost if active
            let speed = if boost.is_boosting {
                enemy.movement_speed * 2.5
            } else {
                enemy.movement_speed
            };
            velocity.linvel = direction * speed;
        } else {
            velocity.linvel = Vec3::ZERO;
        }
    }
} 