use bevy::prelude::*;
use crate::components::Enemy;
use crate::states::GameState;

pub fn check_win_condition(
    enemy_query: Query<&Enemy>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Check if all enemies have been defeated (score <= 0)
    let all_enemies_defeated = enemy_query.iter()
        .all(|enemy| enemy.is_fallen);

    if all_enemies_defeated {
        next_state.set(GameState::WinScreen);
    }
}

pub fn spawn_win_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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
            background_color: Color::rgb(0.96, 0.96, 0.86).into(),
            ..default()
        },
        // Add a component to help with cleanup
        WinScreenMarker,
    ))
    .with_children(|parent| {
        // Win text
        parent.spawn(TextBundle::from_section(
            "You Win!",
            TextStyle {
                font: asset_server.load("fonts/MouldyCheeseRegular-WyMWG.ttf"),
                font_size: 80.0,
                color: Color::WHITE,
            }
        ));

        // Restart button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.56, 0.93, 0.56).into(),
                ..default()
            },
            RestartButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Restart",
                TextStyle {
                    font: asset_server.load("fonts/MouldyCheeseRegular-WyMWG.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                }
            ));
        });
    });
}

pub fn cleanup_win_screen(
    mut commands: Commands,
    win_screen_query: Query<Entity, With<WinScreenMarker>>,
) {
    for entity in win_screen_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_win_screen_input(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<&Interaction, (With<RestartButton>, Changed<Interaction>)>,
) {
    for interaction in interaction_query.iter_mut() {
        if let Interaction::Pressed = interaction {
            next_state.set(GameState::MainMenu);
        }
    }
}

// Marker components for win screen elements
#[derive(Component)]
pub struct WinScreenMarker;

#[derive(Component)]
pub struct RestartButton;
