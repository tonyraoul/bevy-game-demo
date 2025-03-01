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