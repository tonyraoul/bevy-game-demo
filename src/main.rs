use bevy::{
    prelude::*,
    window::WindowMode,
};

fn main() {
    let mut app = App::new();
    
    // Add default plugins with customized window settings
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Demo - Spinning Cube".into(),
            mode: WindowMode::Windowed,
            ..default()
        }),
        ..default()
    }));

    // Add our systems
    app.add_systems(Startup, setup)
        .add_systems(Update, rotate_cube);

    // Run the app
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        SpinningCube,
    ));
}

#[derive(Component)]
struct SpinningCube;

fn rotate_cube(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<SpinningCube>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(1.0 * time.delta_seconds());
        transform.rotate_x(0.5 * time.delta_seconds());
    }
} 