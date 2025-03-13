use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, CollisionGroups, Group, RigidBody, Velocity, LockedAxes, Damping};

pub struct CompoundSphereParams {
    pub base_radius: f32,
    pub head_radius: f32,
    pub head_offset: Vec3,
    pub base_color: Color,
    pub position: Vec3,
    pub is_player: bool,
}

pub fn spawn_compound_sphere(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    params: CompoundSphereParams,
) -> Entity {
    let base_mesh = shape::UVSphere {
        radius: params.base_radius,
        ..default()
    };
    
    let head_mesh = shape::UVSphere {
        radius: params.head_radius,
        ..default()
    };
    
    let collision_group = if params.is_player {
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2)
    } else {
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_2)
    };

    let entity = commands.spawn((
        PbrBundle {
            mesh: meshes.add(base_mesh.into()),
            material: materials.add(StandardMaterial {
                base_color: params.base_color,
                ..default()
            }),
            transform: Transform::from_translation(params.position),
            ..default()
        },
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::compound(vec![
            (Vec3::ZERO, Quat::IDENTITY, Collider::ball(params.base_radius)),
            (params.head_offset, Quat::IDENTITY, Collider::ball(params.head_radius)),
        ]),
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        Damping {
            linear_damping: 0.1,
            angular_damping: 0.5,
        },
        collision_group,
    )).id();

    commands.entity(entity).with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: meshes.add(head_mesh.into()),
            material: materials.add(StandardMaterial {
                base_color: params.base_color,
                ..default()
            }),
            transform: Transform::from_translation(params.head_offset),
            ..default()
        });
    });

    entity
}