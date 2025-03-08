mod menu;
mod player;
mod enemy;
mod boost;
pub mod score;
mod ui;
mod gameover;
mod pause;
mod powerup;
mod win;
mod camera;

pub use menu::*;
pub use player::*;
pub use enemy::*;
pub use boost::*;
pub use ui::*;
pub use gameover::*;
pub use pause::*;
pub use powerup::*;
pub use win::*;
pub use camera::*;

use bevy::prelude::*;

// Create a system set for common game mechanics
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameMechanicsSet {
    Movement,
    EnemyBehavior,
    Collisions,
    Scoring,
}
