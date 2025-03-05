use bevy::prelude::*;
use crate::components::{BearScore, ScoreText};

pub fn update_score_text(
    mut text_query: Query<&mut Text, With<ScoreText>>,
    score_query: Query<(&BearScore, &Transform)>,
) {
    if let Some(mut text) = text_query.iter_mut().next() {
        let mut scores: Vec<(&String, i32)> = score_query
            .iter()
            .filter_map(|(score, transform)| {
                if transform.translation.y > -5.0 {
                    Some((&score.name, score.value))
                } else {
                    None
                }
            })
            .collect();

        scores.sort_by(|a, b| b.1.cmp(&a.1));

        let score_text = scores
            .iter()
            .map(|(name, score)| format!("{}: {}", name, score))
            .collect::<Vec<_>>()
            .join("\n");

        text.sections[0].value = format!("Scores:\n{}", score_text);
    }
}
