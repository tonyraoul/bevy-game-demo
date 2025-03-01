use bevy::{prelude::*, pbr::StandardMaterial};
use bevy_rapier3d::prelude::*;

use crate::states::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(OnEnter(GameState::InGame), setup_game)
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
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ground plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(10.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Collider::cuboid(5.0, 0.0, 5.0),
    ));

    // Cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(1.0).into()),
            material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
    ));
}

fn cleanup_game(
    mut commands: Commands,
    entities: Query<Entity, Or<(With<Camera3d>, With<DirectionalLight>, With<Handle<Mesh>>)>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
} 