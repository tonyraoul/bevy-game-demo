use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.3, 0.5, 0.8); // Cool blue
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.4, 0.6, 0.9); // Lighter blue when hovered
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.2, 0.3, 0.6); // Darker blue when pressed

pub fn get_button_style() -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(200.0),
        height: Val::Px(50.0),
        ..Style::DEFAULT
    }
}

pub fn get_title_text_style(asset_server: &AssetServer) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/MouldyCheeseRegular-WyMWG.ttf"),
        font_size: 64.0,
        color: Color::rgb(1.0, 0.9, 0.2), // Warm yellow color for contrast with winter theme
        ..default()
    }
}

pub fn get_button_text_style(asset_server: &AssetServer) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/MouldyCheeseRegular-WyMWG.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}
