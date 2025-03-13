use bevy::prelude::*;

use crate::states::GameState;
use crate::components::GameSettings;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Settings), setup_settings)
            .add_systems(Update, handle_settings.run_if(in_state(GameState::Settings)))
            .add_systems(OnExit(GameState::Settings), cleanup_settings);
    }
}

#[derive(Component)]
struct SettingsMenu;

fn setup_settings(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Settings UI
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        },
        SettingsMenu,
    ));
}

pub fn handle_settings(
    keys: Res<Input<KeyCode>>,
    _settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

fn cleanup_settings(
    mut commands: Commands,
    settings_query: Query<Entity, With<SettingsMenu>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    // Cleanup settings UI
    for entity in settings_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Cleanup camera
    for entity in camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
