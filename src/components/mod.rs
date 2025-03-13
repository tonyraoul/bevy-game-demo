use bevy::prelude::*;

pub mod menu;
pub mod player;
pub mod ui;
pub mod enemy;
pub mod boost;
pub mod score;
pub mod powerup;
pub mod shape;
pub use shape::*;

pub use menu::*;
pub use player::*;
pub use ui::*;
pub use enemy::*;
pub use boost::*;
pub use score::*;
pub use powerup::*;

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
