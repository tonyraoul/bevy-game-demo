use bevy::prelude::*;

use crate::{
    components::{GameOverScreen, FinalScoreText, GameOverButton, GameOverButtonAction, BearScore},
    styles::*,
    states::GameState,
};

// Debug print to verify the system is running
fn debug_print(message: &str) {
    println!("[GameOver System] {}", message);
}

pub fn spawn_game_over_screen(mut commands: Commands, asset_server: Res<AssetServer>, score_query: Query<&BearScore>) {
    debug_print("Spawning game over screen");
    
    // Find the player's score
    let mut player_score = 0;
    let mut player_name = "Player".to_string();
    
    for score in score_query.iter() {
        if score.name == "Player" {
            player_score = score.value;
            player_name = score.name.clone();
            debug_print(&format!("Found player score: {}", player_score));
            break;
        }
    }

    let game_over_screen = commands
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
                background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            },
            GameOverScreen,
        ))
        .id();

    // Game Over Title
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
            text: Text::from_section("Game Over", get_title_text_style(&asset_server)),
            ..default()
        });
    }).set_parent(game_over_screen);

    // Final Score
    commands.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(400.0),
                height: Val::Px(80.0),
                ..default()
            },
            ..default()
        },
        FinalScoreText,
    )).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                format!("{}'s Final Score: {}", player_name, player_score),
                TextStyle {
                    font_size: 32.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            ..default()
        });
    }).set_parent(game_over_screen);

    // Buttons
    spawn_game_over_button(&mut commands, &asset_server, "Play Again", GameOverButtonAction::Restart, game_over_screen);
    spawn_game_over_button(&mut commands, &asset_server, "Main Menu", GameOverButtonAction::MainMenu, game_over_screen);
}

fn spawn_game_over_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
    action: GameOverButtonAction,
    parent: Entity,
) {
    commands.spawn((
        ButtonBundle {
            style: get_button_style(),
            background_color: NORMAL_BUTTON_COLOR.into(),
            ..default()
        },
        GameOverButton {
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

pub fn handle_game_over_input(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &GameOverButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    debug_print("Handling game over input");
    
    for (interaction, mut color, button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                match button.action {
                    GameOverButtonAction::Restart => {
                        debug_print("Restart button pressed");
                        next_state.set(GameState::InGame);
                    },
                    GameOverButtonAction::MainMenu => {
                        debug_print("Main Menu button pressed");
                        next_state.set(GameState::MainMenu);
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

pub fn cleanup_game_over(
    mut commands: Commands,
    game_over_query: Query<Entity, With<GameOverScreen>>,
) {
    debug_print("Cleaning up game over screen");
    
    for entity in game_over_query.iter() {
        debug_print(&format!("Despawning game over entity: {:?}", entity));
        commands.entity(entity).despawn_recursive();
    }
    
    debug_print("Game over cleanup complete");
}
