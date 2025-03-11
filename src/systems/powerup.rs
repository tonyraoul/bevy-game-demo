use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use crate::components::{
    Player,
    PowerUp,
    PowerUpType,
    ActivePowerUp,
    Enemy,
};

#[derive(Component)]
pub struct PowerUpCoin {
    pub power_type: PowerUpType,
    pub lifetime: Timer,
}

/// Applies active powerup effects to the player.
pub fn apply_powerup_effects(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player, &mut ActivePowerUp)>,
) {
    for (mut transform, mut player, mut active_powerup) in query.iter_mut() {
        // Handle grow powerup
        if let Some(grow) = &mut active_powerup.grow {
            grow.duration.tick(time.delta());
            if grow.duration.finished() {
                player.current_scale = player.base_scale;
                transform.scale = player.current_scale;
                active_powerup.grow = None;
            } else {
                player.current_scale = player.base_scale * 1.5;
                transform.scale = player.current_scale;
            }
        }
        // Handle shrink powerup
        if let Some(shrink) = &mut active_powerup.shrink {
            shrink.duration.tick(time.delta());
            if shrink.duration.finished() {
                player.current_scale = player.base_scale;
                transform.scale = player.current_scale;
                active_powerup.shrink = None;
            } else {
                player.current_scale = player.base_scale * 0.5;
                transform.scale = player.current_scale;
            }
        }
    }
}

/// Spawns a powerup coin randomly above the platform when fewer than two exist.
pub fn spawn_random_powerup_coin(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_timer: Local<Option<Timer>>,
    powerup_query: Query<&PowerUpCoin>,
) {
    let existing_powerups = powerup_query.iter().count();
    // Initialize spawn timer if it's not already set.
    if spawn_timer.is_none() {
        let duration = rand::thread_rng().gen_range(2.0..=6.0);
        *spawn_timer = Some(Timer::from_seconds(duration, TimerMode::Once));
    }

    if let Some(timer) = spawn_timer.as_mut() {
        timer.tick(time.delta());
        if timer.finished() && existing_powerups < 2 {
            let mut rng = rand::thread_rng();
            let power_type = if rng.gen_bool(0.5) {
                PowerUpType::Grow
            } else {
                PowerUpType::Shrink
            };
            // Spawn coin at a random position above the platform.
            let x = rng.gen_range(-5.0..5.0);
            let y = rng.gen_range(-5.0..5.0);
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(shape::Box::new(0.5, 0.5, 0.5).into()),
                    material: materials.add(StandardMaterial {
                        base_color: match power_type {
                            PowerUpType::Grow => Color::rgb(1.0, 0.8, 0.0),
                            PowerUpType::Shrink => Color::rgb(0.0, 0.8, 1.0),
                        },
                        metallic: 1.0,
                        perceptual_roughness: 0.1,
                        ..default()
                    }),
                    transform: Transform::from_xyz(x, 6.0, y),
                    ..default()
                },
                PowerUpCoin {
                    power_type,
                    lifetime: Timer::from_seconds(10.0, TimerMode::Once),
                },
                // Set the coin to be fixed so it doesn't fall.
                RigidBody::Fixed,
                Collider::cuboid(0.25, 0.25, 0.25),
                CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_2),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
            ));

            // Reset timer with a new random duration.
            let duration = rng.gen_range(4.0..=8.0);
            *spawn_timer = Some(Timer::from_seconds(duration, TimerMode::Once));
        }
    }
}

/// Handles collision events to collect powerup coins.
pub fn collect_powerup_coin(
    mut commands: Commands,
    mut player_query: Query<&mut ActivePowerUp, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    coin_query: Query<(Entity, &PowerUpCoin)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    let mut coins_to_despawn = Vec::new();
    let mut powerup_to_apply = None;

    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            let (coin_entity, coin) = if let Ok((entity, coin)) = coin_query.get(*e1) {
                (entity, coin)
            } else if let Ok((entity, coin)) = coin_query.get(*e2) {
                (entity, coin)
            } else {
                continue;
            };

            let other_entity = if coin_entity == *e1 { *e2 } else { *e1 };
            if enemy_query.get(other_entity).is_ok() || player_query.get(other_entity).is_ok() {
                coins_to_despawn.push(coin_entity);
                if player_query.get(other_entity).is_ok() {
                    powerup_to_apply = Some(coin.power_type);
                }
            }
        }
    }

    for coin_entity in coins_to_despawn {
        commands.entity(coin_entity).despawn();
    }

    if let Some(power_type) = powerup_to_apply {
        let mut active_powerup = player_query.single_mut();
        apply_powerup_effect(&mut active_powerup, power_type);
    }
}

/// Applies the powerup effect to the player's active powerup state.
fn apply_powerup_effect(active_powerup: &mut ActivePowerUp, power_type: PowerUpType) {
    match power_type {
        PowerUpType::Grow => {
            if active_powerup.grow.is_none() {
                active_powerup.grow = Some(PowerUp::new(PowerUpType::Grow, 6.0));
            }
        },
        PowerUpType::Shrink => {
            if active_powerup.shrink.is_none() {
                active_powerup.shrink = Some(PowerUp::new(PowerUpType::Shrink, 6.0));
            }
        },
    }
}

/// Removes expired powerup coins.
pub fn remove_expired_powerup_coins(
    mut commands: Commands,
    time: Res<Time>,
    mut coin_query: Query<(Entity, &mut PowerUpCoin)>,
) {
    for (entity, mut coin) in coin_query.iter_mut() {
        coin.lifetime.tick(time.delta());
        if coin.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
