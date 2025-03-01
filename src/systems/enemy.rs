use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::components::{Enemy, EnemyState, Player, EnergyBoost, PLATFORM_HEIGHT, BearScore};

const SPAWN_POSITIONS: [(f32, f32); 4] = [
    (-8.0, -8.0),
    (-8.0, 8.0),
    (8.0, -8.0),
    (8.0, 8.0),
];

const FALL_THRESHOLD: f32 = -5.0;
const RESPAWN_POSITION: Vec3 = Vec3::new(0.0, PLATFORM_HEIGHT + 2.0, 0.0);

// Physics constants
const BASE_MOVEMENT_FORCE: f32 = 15.0;
const MAX_SPEED: f32 = 6.0;
const FRICTION: f32 = 0.95;
const CHASE_DISTANCE: f32 = 10.0;

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
            linear_damping: 0.5,
            angular_damping: 1.0,
        },
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_2),  // Enemy can collide with environment and other bears
    ));
}

pub fn handle_enemy_falls(
    mut enemy_query: Query<(&mut Enemy, &mut Transform, &mut Velocity, &mut BearScore)>,
    time: Res<Time>,
) {
    for (mut enemy, mut transform, mut velocity, mut score) in enemy_query.iter_mut() {
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

        if enemy.state_timer.just_finished() {
            // Randomly change state
            if rng.gen_bool(0.3) {
                // Change to chase if player is nearby
                let distance_to_player = transform.translation.distance(player_pos);
                enemy.state = match enemy.state {
                    EnemyState::Patrol if distance_to_player < CHASE_DISTANCE => EnemyState::Chase,
                    EnemyState::Chase if distance_to_player > CHASE_DISTANCE * 1.5 => EnemyState::Patrol,
                    EnemyState::Patrol => EnemyState::Patrol,
                    EnemyState::Chase => EnemyState::Chase,
                    EnemyState::Fallen => EnemyState::Patrol,
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
            EnemyState::Fallen => continue,
        };

        // Calculate movement direction
        let direction = (target_pos - transform.translation).normalize();
        
        // Only move if not too close to target
        if transform.translation.distance(target_pos) > 2.0 {
            // Apply movement force
            let force = if boost.is_boosting {
                BASE_MOVEMENT_FORCE * 2.5
            } else {
                BASE_MOVEMENT_FORCE
            };
            
            velocity.linvel += direction * force * time.delta_seconds();

            // Apply friction
            velocity.linvel *= FRICTION;

            // Clamp maximum speed
            let speed = velocity.linvel.length();
            if speed > MAX_SPEED {
                velocity.linvel = velocity.linvel.normalize() * MAX_SPEED;
            }
        } else {
            velocity.linvel *= FRICTION;
        }
    }
} 