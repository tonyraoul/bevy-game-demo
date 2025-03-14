use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, CollisionGroups, Group, RigidBody, Velocity, LockedAxes, Damping};

pub struct DuckParams {
    pub body_radius: f32,
    pub head_radius: f32,
    pub bill_length: f32,
    pub body_offset: Vec3,
    pub head_offset: Vec3,
    pub bill_offset: Vec3,
    pub base_color: Color,
    pub bill_color: Color,
    pub position: Vec3,
    pub is_player: bool,
}

pub fn spawn_duck(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    params: DuckParams,
) -> Entity {
    let body_mesh = shape::UVSphere {
        radius: params.body_radius,
        ..default()
    };
    
    let head_mesh = shape::UVSphere {
        radius: params.head_radius,
        ..default()
    };

    let bill_mesh = shape::Box {
        min_x: 0.0,
        max_x: params.bill_length,
        min_y: -params.head_radius / 2.0,
        max_y: params.head_radius / 2.0,
        min_z: -params.head_radius / 2.0,
        max_z: params.head_radius / 2.0,
    };
    
    let collision_group = if params.is_player {
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2)
    } else {
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_2)
    };

    let entity = commands.spawn((
        PbrBundle {
            mesh: meshes.add(body_mesh.into()),
            material: materials.add(StandardMaterial {
                base_color: params.base_color,
                ..default()
            }),
            transform: Transform::from_translation(params.position + params.body_offset),
            ..default()
        },
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::compound(vec![
            (params.body_offset, Quat::IDENTITY, Collider::ball(params.body_radius)),
            (params.body_offset + params.head_offset, Quat::IDENTITY, Collider::ball(params.head_radius)),
            (params.body_offset + params.head_offset + params.bill_offset, Quat::IDENTITY, Collider::cuboid(params.bill_length / 2.0, params.head_radius / 2.0, params.head_radius / 2.0)),
        ]),
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        Damping {
            linear_damping: 0.1,
            angular_damping: 0.5,
        },
        collision_group,
    )).id();

    commands.entity(entity).with_children(|parent| {
        // Spawn head
        parent.spawn(PbrBundle {
            mesh: meshes.add(head_mesh.into()),
            material: materials.add(StandardMaterial {
                base_color: params.base_color,
                ..default()
            }),
            transform: Transform::from_translation(params.body_offset + params.head_offset),
            ..default()
        });

        // Spawn bill
        parent.spawn(PbrBundle {
            mesh: meshes.add(bill_mesh.into()),
            material: materials.add(StandardMaterial {
                base_color: params.bill_color,
                ..default()
            }),
            transform: Transform::from_translation(params.body_offset + params.head_offset + params.bill_offset),
            ..default()
        });
    });

    entity
}