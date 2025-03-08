use bevy::prelude::*;
use bevy::app::AppExit;

use crate::{
    components::{PauseMenu, PauseButton, PauseButtonAction, PauseState},
    styles::*,
    states::GameState,
};

// Removed debug_print function

pub fn toggle_pause(
    keyboard_input: Res<Input<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut pause_state: ResMut<PauseState>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::InGame => {
                // Set flags to indicate we're transitioning to pause
                pause_state.transitioning_to_pause = true;
                pause_state.was_paused = true;
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                // Clear transitioning flag when resuming, but keep was_paused true
                pause_state.transitioning_to_pause = false;
                next_state.set(GameState::InGame);
            }
            _ => {}
        }
    }
}

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pause_menu = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::rgb(0.96, 0.96, 0.86).into(),
                ..default()
            },
            PauseMenu,
        ))
        .id();

    // Pause Title
    commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Px(400.0),
            height: Val::Px(120.0),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section("Paused", get_title_text_style(&asset_server)),
            ..default()
        });
    }).set_parent(pause_menu);

    // Buttons
    spawn_pause_button(&mut commands, &asset_server, "Resume", PauseButtonAction::Resume, pause_menu);
    spawn_pause_button(&mut commands, &asset_server, "Main Menu", PauseButtonAction::MainMenu, pause_menu);
    spawn_pause_button(&mut commands, &asset_server, "Quit Game", PauseButtonAction::Quit, pause_menu);
}

fn spawn_pause_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
    action: PauseButtonAction,
    parent: Entity,
) {
    commands.spawn((
        ButtonBundle {
            style: get_button_style(),
            background_color: NORMAL_BUTTON_COLOR.into(),
            ..default()
        },
        PauseButton {
            action: action.clone(),
        },
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(text, get_button_text_style(asset_server)),
            ..default()
        });
    })
    .set_parent(parent);
}

pub fn handle_pause_input(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &PauseButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut pause_state: ResMut<PauseState>,
    mut app_exit_events: EventWriter<AppExit>,
    mut commands: Commands,
    query: Query<Entity, Without<Camera>>,
    camera_query: Query<Entity, With<Camera>>,
    window_query: Query<Entity, With<Window>>,
) {
    for (interaction, mut color, button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                match button.action {
                    PauseButtonAction::Resume => {
                        // Clear transitioning flag when resuming, but keep was_paused true
                        pause_state.transitioning_to_pause = false;
                        next_state.set(GameState::InGame);
                    },
                    PauseButtonAction::MainMenu => {
                        // Reset both flags when going to main menu
                        pause_state.transitioning_to_pause = false;
                        pause_state.was_paused = false;
                        
                        // Clean up all game entities except camera and window
                        for entity in query.iter() {
                            if camera_query.get(entity).is_err() && window_query.get(entity).is_err() {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                        
                        next_state.set(GameState::MainMenu);
                    },
                    PauseButtonAction::Quit => {
                        app_exit_events.send(AppExit);
                    },
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn cleanup_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    for entity in pause_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
