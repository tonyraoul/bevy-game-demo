// Removed unused import
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct GameHud;

#[derive(Component)]
pub struct BoostIndicator;

#[derive(Component)]
pub struct BoostText;

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

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct PauseButton {
    pub action: PauseButtonAction,
}

#[derive(Clone)]
pub enum PauseButtonAction {
    Resume,
    MainMenu,
    Quit,
}
