use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct GameHud;

#[derive(Component)]
pub struct BoostIndicator;

#[derive(Component)]
pub struct GameOverScreen;

#[derive(Component)]
pub struct FinalScoreText;

#[derive(Component)]
pub struct GameOverButton {
    pub action: GameOverButtonAction,
}

#[derive(Clone)]
pub enum GameOverButtonAction {
    Restart,
    MainMenu,
}
