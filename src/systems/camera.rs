use bevy::prelude::*;
use crate::components::{Player, Enemy};

pub fn update_camera_position(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>, Without<Enemy>)>,
    entity_query: Query<&GlobalTransform, Or<(With<Player>, With<Enemy>)>>,
    time: Res<Time>,
) {
    // Platform height where entities should be considered
    const PLATFORM_HEIGHT: f32 = 5.0;
    const PLATFORM_TOLERANCE: f32 = 2.0;
    const CAMERA_SMOOTHING: f32 = 0.001; // Lower value means slower camera movement

    // If no camera or no entities, do nothing
    if camera_query.is_empty() || entity_query.is_empty() {
        return;
    }

    // Filter out entities that are falling (below platform height)
    let active_entity_positions: Vec<Vec3> = entity_query.iter()
        .map(|transform| transform.translation())
        .filter(|pos| pos.y >= PLATFORM_HEIGHT - PLATFORM_TOLERANCE)
        .collect();

    // If no active entities, do nothing
    if active_entity_positions.is_empty() {
        return;
    }

    // Calculate the center point of active entities
    let center = active_entity_positions.iter().sum::<Vec3>() / active_entity_positions.len() as f32;

    // Calculate the maximum distance from the center to determine camera zoom/height
    let max_distance = active_entity_positions
        .iter()
        .map(|&pos| Vec3::distance(pos, center))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(10.0);

    // Adjust camera position based on entities' positions
    // Raise the camera height to provide a good overview
    let camera_height = max_distance.max(10.0) + 10.0;
    let camera_distance = max_distance.max(10.0) + 15.0;

    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        // Smoothly interpolate camera position
        let target_translation = Vec3::new(
            center.x, 
            camera_height, 
            center.z + camera_distance
        );
        
        camera_transform.translation = camera_transform.translation.lerp(
            target_translation, 
            time.delta_seconds() * CAMERA_SMOOTHING
        );
        
        camera_transform.look_at(center, Vec3::Y);
    }
}
