use bevy::prelude::*;
use crate::components::{BearScore, ScoreText};

pub fn update_score_text(
    mut text_query: Query<&mut Text, With<ScoreText>>,
    score_query: Query<(&BearScore, &Transform)>,
) {
    for mut text in text_query.iter_mut() {
        let mut scores = score_query
            .iter()
            .map(|(score, transform)| (score.name.clone(), score.value, transform.translation.y))
            .collect::<Vec<_>>();
        
        // Sort by score value (descending) and filter out fallen bears
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
