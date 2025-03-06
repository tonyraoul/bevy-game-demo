use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct MainMenu;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct SettingsMenu;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct MenuButton {
    pub action: MenuButtonAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuButtonAction {
    Play,
    Settings,
    Quit,
    Back,
    Secret,
}
