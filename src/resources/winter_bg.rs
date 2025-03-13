use bevy::{
    prelude::*,
    time::Time,
};

// Component for the winter background
#[derive(Component)]
pub struct WinterBackground;

// Component for snowflakes
#[derive(Component)]
pub struct Snowflake {
    speed: f32,
    drift: f32,
    time_offset: f32,
}

// Plugin for winter background
pub struct WinterBackgroundPlugin;

impl Plugin for WinterBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_snowflakes);
    }
}

// System to spawn the winter background
pub fn spawn_winter_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a blue gradient background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.4, 0.8), // Deep blue
                custom_size: Some(Vec2::new(1000.0, 1000.0)), // Large enough to cover screen
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
            ..default()
        },
        WinterBackground,
    ));
    
    // Spawn a set of snowflakes with predefined positions
    spawn_predefined_snowflakes(&mut commands);
}

// System to clean up the winter background
pub fn cleanup_winter_background(
    mut commands: Commands,
    background_query: Query<Entity, With<WinterBackground>>,
    snowflake_query: Query<Entity, With<Snowflake>>,
) {
    // Remove background
    for entity in background_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Remove all snowflakes
    for entity in snowflake_query.iter() {
        commands.entity(entity).despawn();
    }
}

// System to update snowflakes
fn update_snowflakes(
    time: Res<Time>,
    mut snowflake_query: Query<(&mut Transform, &Snowflake)>,
    window_query: Query<&Window>,
) {
    if let Ok(window) = window_query.get_single() {
        let window_height = window.height();
        let window_width = window.width();
        
        for (mut transform, snowflake) in snowflake_query.iter_mut() {
            // Move snowflake down
            transform.translation.y -= snowflake.speed * time.delta_seconds();
            
            // Add some horizontal drift using sine wave
            let drift_amount = (time.elapsed_seconds() + snowflake.time_offset) * 1.5;
            transform.translation.x += snowflake.drift * f32::sin(drift_amount) * time.delta_seconds();
            
            // Rotate the snowflake gently
            transform.rotate_z(0.1 * time.delta_seconds());
            
            // If snowflake is below the screen, reset to top
            if transform.translation.y < -window_height / 2.0 - 20.0 {
                transform.translation.y = window_height / 2.0 + 20.0;
                // Keep the same x position for simplicity
            }
            
            // If snowflake is off the sides, wrap around
            if transform.translation.x < -window_width / 2.0 - 20.0 {
                transform.translation.x = window_width / 2.0 + 20.0;
            } else if transform.translation.x > window_width / 2.0 + 20.0 {
                transform.translation.x = -window_width / 2.0 - 20.0;
            }
        }
    }
}

// Helper function to spawn predefined snowflakes
fn spawn_predefined_snowflakes(commands: &mut Commands) {
    // Create a set of predefined snowflakes with different properties
    let snowflake_data = [
        // x, y, size, speed, drift, time_offset
        (-300.0, 300.0, 10.0, 50.0, 5.0, 0.0),
        (-250.0, 250.0, 8.0, 40.0, -3.0, 1.0),
        (-200.0, 350.0, 12.0, 60.0, 7.0, 2.0),
        (-150.0, 200.0, 7.0, 45.0, -4.0, 3.0),
        (-100.0, 280.0, 9.0, 55.0, 6.0, 4.0),
        (-50.0, 320.0, 11.0, 35.0, -8.0, 5.0),
        (0.0, 270.0, 10.0, 50.0, 4.0, 6.0),
        (50.0, 330.0, 8.0, 65.0, -5.0, 7.0),
        (100.0, 290.0, 13.0, 40.0, 9.0, 8.0),
        (150.0, 310.0, 7.0, 55.0, -3.0, 9.0),
        (200.0, 260.0, 9.0, 45.0, 6.0, 10.0),
        (250.0, 340.0, 11.0, 60.0, -7.0, 11.0),
        (300.0, 280.0, 10.0, 50.0, 5.0, 12.0),
        // Second row
        (-320.0, 200.0, 9.0, 55.0, -4.0, 13.0),
        (-270.0, 230.0, 11.0, 40.0, 8.0, 14.0),
        (-220.0, 180.0, 7.0, 60.0, -3.0, 15.0),
        (-170.0, 210.0, 10.0, 45.0, 6.0, 16.0),
        (-120.0, 190.0, 8.0, 50.0, -7.0, 17.0),
        (-70.0, 220.0, 12.0, 35.0, 5.0, 18.0),
        (-20.0, 170.0, 9.0, 55.0, -4.0, 19.0),
        (30.0, 240.0, 11.0, 40.0, 9.0, 20.0),
        (80.0, 200.0, 7.0, 60.0, -3.0, 21.0),
        (130.0, 230.0, 10.0, 45.0, 6.0, 22.0),
        (180.0, 190.0, 8.0, 50.0, -7.0, 23.0),
        (230.0, 220.0, 12.0, 35.0, 5.0, 24.0),
        (280.0, 170.0, 9.0, 55.0, -4.0, 25.0),
        // Third row
        (-290.0, 100.0, 11.0, 40.0, 8.0, 26.0),
        (-240.0, 130.0, 7.0, 60.0, -3.0, 27.0),
        (-190.0, 80.0, 10.0, 45.0, 6.0, 28.0),
        (-140.0, 110.0, 8.0, 50.0, -7.0, 29.0),
        (-90.0, 90.0, 12.0, 35.0, 5.0, 30.0),
        (-40.0, 120.0, 9.0, 55.0, -4.0, 31.0),
        (10.0, 70.0, 11.0, 40.0, 9.0, 32.0),
        (60.0, 140.0, 7.0, 60.0, -3.0, 33.0),
        (110.0, 100.0, 10.0, 45.0, 6.0, 34.0),
        (160.0, 130.0, 8.0, 50.0, -7.0, 35.0),
        (210.0, 80.0, 12.0, 35.0, 5.0, 36.0),
        (260.0, 110.0, 9.0, 55.0, -4.0, 37.0),
        (310.0, 90.0, 11.0, 40.0, 8.0, 38.0),
    ];
    
    for (x, y, size, speed, drift, time_offset) in snowflake_data.iter() {
        // Create a simple circle for the snowflake
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(*size, *size)),
                    color: Color::rgba(1.0, 1.0, 1.0, 0.7),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(*x, *y, -5.0)),
                ..default()
            },
            Snowflake {
                speed: *speed,
                drift: *drift,
                time_offset: *time_offset,
            },
        ));
    }
}
