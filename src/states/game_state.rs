use bevy::state::state::States;

#[derive(Clone, Debug, Eq, Hash, PartialEq, States)]
pub enum GameState {
  MainMenu,
  InGame,
  Paused,
  GameOver,
}
