use bevy::prelude::*;

use crate::components::{GameHud, ScoreText};
use crate::systems::Score;

pub fn spawn_hud(mut commands: Commands) {
    // Spawn HUD root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
            GameHud,
        ))
        .with_children(|parent| {
            // Score text
            parent.spawn((
                TextBundle::from_section(
                    "Score: 10",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                }),
                ScoreText,
            ));
        });
}

pub fn update_score_text(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Score: {}", score.value);
    }
}

pub fn cleanup_hud(
    mut commands: Commands,
    query: Query<Entity, With<GameHud>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
} 