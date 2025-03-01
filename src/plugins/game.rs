use bevy::{prelude::*, pbr::StandardMaterial};
use bevy_rapier3d::prelude::*;

use crate::{states::GameState, systems::*};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(OnEnter(GameState::InGame), (setup_game, spawn_player))
            .add_systems(
                Update,
                (
                    player_movement,
                    check_fall,
                )
                    .run_if(in_state(GameState::InGame))
            )
            .add_systems(OnExit(GameState::InGame), cleanup_game);
    }
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-15.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Platform - Main surface
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(20.0, 1.0, 20.0).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                metallic: 0.0,
                perceptual_roughness: 1.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Collider::cuboid(10.0, 0.5, 10.0),
        RigidBody::Fixed,
    ));

    // Platform - Edge highlight
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(20.2, 0.2, 20.2).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.0),
            emissive: Color::rgb(0.5, 0.4, 0.0),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.6, 0.0),
        ..default()
    });
}

fn cleanup_game(
    mut commands: Commands,
    entities: Query<Entity, Or<(With<Camera3d>, With<DirectionalLight>, With<Handle<Mesh>>)>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
} 