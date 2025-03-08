use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::states::GameState;
use crate::components::{GameSettings, PauseState};
use crate::systems::{
    player_movement,
    check_fall,
    enemy_behavior,
    handle_enemy_falls,
    spawn_enemies,
    handle_boost,
    handle_ai_boost,
    update_boost_indicator,
    spawn_player,
    spawn_hud,
    spawn_game_over_screen,
    handle_game_over_input,
    cleanup_game_over,
    toggle_pause,
    spawn_pause_menu,
    handle_pause_input,
    cleanup_pause_menu,
    apply_powerup_effects,
    spawn_random_powerup_coin,
    collect_powerup_coin,
    remove_expired_powerup_coins,
    check_win_condition,
    spawn_win_screen,
    cleanup_win_screen,
    handle_win_screen_input,
    update_score_text,
};

pub struct GamePlugin;

fn cleanup_game(
    mut commands: Commands,
    query: Query<Entity, (Without<Camera>, Without<Window>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn conditional_cleanup_game(
    mut commands: Commands,
    query: Query<Entity, (Without<Camera>, Without<Window>)>,
    pause_state: Res<PauseState>,
    next_state: Res<NextState<GameState>>,
) {
    // Only clean up if we're not transitioning to the pause state
    if !pause_state.transitioning_to_pause {
        // Check if we're transitioning to a state other than Paused
        if let Some(state) = next_state.0.as_ref() {
            if *state != GameState::Paused {
                cleanup_game(commands, query);
            }
        } else {
            // If no next state is set, clean up anyway
            cleanup_game(commands, query);
        }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameSettings>()
            .init_resource::<PauseState>()
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            // Add a system set that runs when entering InGame from a state other than Paused
            .add_systems(
                OnEnter(GameState::InGame),
                (setup_game, spawn_player, spawn_hud, spawn_enemies)
                    .run_if(not(run_if_resuming_from_pause))
            )
            .add_systems(Update, (
                handle_boost,
                handle_ai_boost,
                update_boost_indicator,
                player_movement,
                enemy_behavior,
                check_fall,
                handle_enemy_falls,
                update_score_text.after(handle_enemy_falls),
                apply_powerup_effects,
                spawn_random_powerup_coin,
                collect_powerup_coin,
                remove_expired_powerup_coins,
                toggle_pause,
                check_win_condition,
            ).run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), conditional_cleanup_game)
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(Update, handle_pause_input.run_if(in_state(GameState::Paused)))
            .add_systems(OnExit(GameState::Paused), cleanup_pause_menu)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_screen)
            .add_systems(Update, handle_game_over_input.run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
            .add_systems(OnEnter(GameState::WinScreen), spawn_win_screen)
            .add_systems(Update, handle_win_screen_input.run_if(in_state(GameState::WinScreen)))
            .add_systems(OnExit(GameState::WinScreen), cleanup_win_screen);
    }
}

// Function to check if we're resuming from pause
fn run_if_resuming_from_pause(
    pause_state: Res<PauseState>,
) -> bool {
    // If was_paused is true, we're resuming from pause
    pause_state.was_paused
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
    let platform_radius = 10.0;
    let platform_height = 1.0;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cylinder {
                radius: platform_radius,
                height: platform_height,
                ..default()
            }.into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cylinder(platform_height / 2.0, platform_radius),
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2),
    ));

    // Platform edge highlight
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cylinder {
                radius: platform_radius + 0.1,
                height: 0.2,
                ..default()
            }.into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.6, 0.2),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 5.6, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cylinder(0.1, platform_radius + 0.1),
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2),
    ));
}
