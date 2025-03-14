use bevy::prelude::*;

use crate::components::{
    GameHud, 
    ScoreText, 
    BoostIndicator, 
    BoostText, 
    DuckScore
};

pub fn spawn_hud(mut commands: Commands) {
    // Root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            GameHud,
        ))
        .with_children(|parent| {
            // Left side - Scores
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Score text
                    parent.spawn((
                        TextBundle::from_section(
                            "Scores:",
                            TextStyle {
                                font_size: 24.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        ScoreText,
                    ));
                });

            // Right side - Boost indicator
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(250.0), // Increased width to accommodate text
                        height: Val::Px(20.0),
                        position_type: PositionType::Absolute,
                        right: Val::Px(10.0),
                        top: Val::Px(10.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Boost text
                    parent.spawn(
                        TextBundle::from_section(
                            "Boost:",
                            TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                ..default()
                            }
                        )
                        .with_style(Style {
                            margin: UiRect::right(Val::Px(10.0)),
                            ..default()
                        })
                    ).insert(BoostText);

                    // Boost background
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(20.0),
                            ..default()
                        },
                        background_color: Color::rgb(0.96, 0.96, 0.86).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Boost fill
                        parent.spawn(
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                background_color: Color::rgb(0.68, 0.85, 0.90).into(),
                                ..default()
                            }
                        ).insert(BoostIndicator);
                    });
                });
        });
}

pub fn update_score_text(
    mut text_query: Query<&mut Text, With<ScoreText>>,
    score_query: Query<(&DuckScore, &Transform)>,
) {
    for mut text in text_query.iter_mut() {
        let mut scores = score_query
            .iter()
            .map(|(score, transform)| (score.name.clone(), score.value, transform.translation.y))
        .collect::<Vec<_>>();
        
        // Sort by score value (descending) and filter out fallen ducks
        scores.sort_by(|a, b| b.1.cmp(&a.1));
        scores.retain(|(_name, _score, y)| *y > -5.0);

        // Update text
        text.sections[0].value = format!(
            "Scores:\n{}",
            scores
                .iter()
                .map(|(name, score, _)| format!("{}: {}", name, score))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

/// Cleans up the game HUD, typically used during state transitions
/// Removes all entities associated with the game HUD
pub fn cleanup_hud(mut commands: Commands, hud_query: Query<Entity, With<GameHud>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
