use bevy::prelude::*;
use crate::systems::{spawn_main_menu, handle_menu_buttons};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_menu)
            .add_systems(Update, handle_menu_buttons);
    }
} 