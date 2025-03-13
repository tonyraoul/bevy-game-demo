mod menu;
mod player;
mod boost;
pub mod score;
mod ui;
mod gameover;
mod pause;
mod powerup;
mod win;
mod camera;

mod enemy_spawning;
mod enemy_falling;
mod enemy_movement;

pub use menu::*;
pub use player::*;
pub use boost::*;
pub use ui::*;
pub use gameover::*;
pub use pause::*;
pub use powerup::*;
pub use win::*;
pub use camera::*;

pub use enemy_spawning::spawn_enemies;
pub use enemy_falling::handle_enemy_falls;
pub use enemy_movement::enemy_behavior;

use bevy::prelude::*;

// Create a system set for common game mechanics
/// System set for organizing and configuring game mechanics systems
/// Even though variants appear unused, they are used for system configuration
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub enum GameMechanicsSet {
    Movement,
    EnemyBehavior,
    Collisions,
    Scoring,
}
