use bevy::prelude::*;
use crate::components::BearScore;

pub fn update_score_text(
    query: Query<&BearScore>,
    mut text_query: Query<&mut Text>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        let mut score_text = String::new();
        for score in query.iter() {
            score_text.push_str(&format!("{}: {}\n", score.name, score.value));
        }
        text.sections[0].value = score_text;
    }
} 