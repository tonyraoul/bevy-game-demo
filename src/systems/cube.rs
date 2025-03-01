use bevy::prelude::*;
use crate::components::SpinningCube;

pub fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.2, 0.2),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        SpinningCube::default(),
    ));
}

pub fn setup_camera_and_light(mut commands: Commands) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

pub fn rotate_cube(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &SpinningCube)>,
) {
    for (mut transform, cube) in &mut query {
        transform.rotate_y(cube.rotation_speed_y * time.delta_seconds());
        transform.rotate_x(cube.rotation_speed_x * time.delta_seconds());
    }
} 