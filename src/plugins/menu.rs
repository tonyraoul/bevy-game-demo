use bevy::prelude::*;

use crate::{
    systems::{spawn_main_menu, handle_menu_buttons, cleanup_menu},
    states::GameState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(Update, handle_menu_buttons.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
    }
} 