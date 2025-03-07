use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.56, 0.93, 0.56);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn get_button_style() -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(200.0),
        height: Val::Px(50.0),
        ..Style::DEFAULT
    }
}

pub fn get_title_text_style(_asset_server: &AssetServer) -> TextStyle {
    TextStyle {
        font_size: 64.0,
        color: Color::WHITE,
        ..default()
    }
}

pub fn get_button_text_style(_asset_server: &AssetServer) -> TextStyle {
    TextStyle {
        font_size: 32.0,
        color: Color::WHITE,
        ..default()
    }
}
