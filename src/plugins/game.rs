use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::GameState;
use crate::components::GameSettings;
use crate::systems::{
    player_movement,
    enemy_behavior,
    check_fall,
    handle_enemy_falls,
    handle_boost,
    handle_ai_boost,
    update_boost_indicator,
    spawn_player,
    spawn_hud,
    spawn_enemies,
};
use crate::systems::score::update_score_text;
use crate::plugins::settings::handle_settings;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameSettings>()
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(OnEnter(GameState::InGame), (setup_game, spawn_player, spawn_hud, spawn_enemies))
            .add_systems(Update, (
                handle_settings,
                handle_boost,
                handle_ai_boost,
                update_boost_indicator,
                player_movement,
                enemy_behavior,
                check_fall,
                handle_enemy_falls,
                update_score_text.after(handle_enemy_falls),
            ).run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), cleanup_game);
    }
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-15.0, 20.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-15.0, 20.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Platform
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(20.0, 1.0, 20.0).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.5, 10.0),
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2),
    ));

    // Platform edge highlight
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(20.2, 0.2, 20.2).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.6, 0.2),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 5.6, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(10.1, 0.1, 10.1),
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2),
    ));
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, Without<Camera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
} 