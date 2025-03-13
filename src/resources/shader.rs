use bevy::{
    prelude::*,
    render::{
        render_resource::{ShaderType, AsBindGroup},
        extract_resource::ExtractResource,
    },
    sprite::Material2dPlugin,
};

// A simple plugin for the winter background
pub struct WinterBackgroundPlugin;

impl Plugin for WinterBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_time)
           .init_resource::<WinterTime>();
    }
}

// A simple resource to track time for the shader
#[derive(Resource, Default, Clone, ExtractResource)]
pub struct WinterTime {
    pub seconds: f32,
}

// System to update the time
fn update_time(time: Res<Time>, mut winter_time: ResMut<WinterTime>) {
    winter_time.seconds = time.elapsed_seconds();
}

// System to spawn a fullscreen quad with our shader
pub fn spawn_winter_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a fullscreen quad
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1000.0, 1000.0)), // Make it large enough to cover the screen
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)), // Behind UI
        texture: asset_server.load("shaders/winter_menu_bg.wgsl"),
        ..default()
    });
}

// System to clean up the background
pub fn cleanup_winter_background(mut commands: Commands, query: Query<Entity, With<Sprite>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
