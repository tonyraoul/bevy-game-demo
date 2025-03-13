use bevy::prelude::*;

use crate::{
    systems::{spawn_main_menu, handle_menu_buttons, cleanup_menu, spawn_settings_menu},
    states::GameState,
    resources::{WinterBackgroundPlugin, spawn_winter_background, cleanup_winter_background},
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(WinterBackgroundPlugin)
            .add_systems(OnEnter(GameState::MainMenu), (spawn_main_menu, spawn_winter_background))
            .add_systems(OnEnter(GameState::Settings), (spawn_settings_menu, spawn_winter_background))
            .add_systems(Update, handle_menu_buttons.run_if(in_state(GameState::MainMenu).or_else(in_state(GameState::Settings))))
            .add_systems(OnExit(GameState::MainMenu), (cleanup_menu, cleanup_winter_background))
            .add_systems(OnExit(GameState::Settings), cleanup_menu);
    }
}
