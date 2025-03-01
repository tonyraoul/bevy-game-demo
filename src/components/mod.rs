use bevy::prelude::*;

mod menu;
mod player;
mod ui;
mod enemy;
mod boost;
mod score;

pub use menu::*;
pub use player::*;
pub use ui::*;
pub use enemy::*;
pub use boost::*;
pub use score::*;

#[derive(Resource)]
pub struct GameSettings {
    pub paused: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            paused: false,
        }
    }
}

#[derive(Resource)]
pub struct PauseState {
    pub transitioning_to_pause: bool,
    pub was_paused: bool,
}

impl Default for PauseState {
    fn default() -> Self {
        Self {
            transitioning_to_pause: false,
            was_paused: false,
        }
    }
}
