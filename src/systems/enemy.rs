use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::components::{Enemy, EnemyState, Player, EnergyBoost, PLATFORM_HEIGHT, BearScore};

const SPAWN_POSITIONS: [(f32, f32); 1] = [
    // (-8.0, -8.0),
    // (-8.0, 8.0),
    // (8.0, -8.0),
    (8.0, 8.0),
];

const FALL_THRESHOLD: f32 = -5.0;
const RESPAWN_POSITION: Vec3 = Vec3::new(0.0, PLATFORM_HEIGHT + 2.0, 0.0);

// Physics constants
const BASE_MOVEMENT_FORCE: f32 = 15.0;
const MAX_SPEED: f32 = 6.0;
const FRICTION: f32 = 0.95;
const CHASE_DISTANCE: f32 = 10.0;
const FALL_ACCELERATION: f32 = 30.0;  // Additional downward force when falling

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
            println!("Enemy {} has fallen! Position: {:?}", score.name, transform.translation);
            // Enemy has fallen
            enemy.is_fallen = true;
            enemy.state = EnemyState::Fallen;
            enemy.respawn_timer.reset();
            score.value -= 1;  // Deduct a point
            
            // Hide the enemy far below the platform
            transform.translation.y = FALL_THRESHOLD - 20.0;
            velocity.linvel = Vec3::ZERO;
            velocity.angvel = Vec3::ZERO;
        }

        // Handle respawn timer for fallen enemies
        if enemy.is_fallen {
            if enemy.respawn_timer.tick(time.delta()).finished() {
                println!("Enemy {} respawning!", score.name);
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
