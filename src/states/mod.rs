pub mod game_active;
pub mod game_lost;
pub mod game_start;
pub mod main_menu;

use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{
  game_active::GameActiveState, game_lost::GameLostState, game_start::GameStartState,
  main_menu::MainMenuState,
};
pub struct GameStatesPlugin;

impl PluginGroup for GameStatesPlugin {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      .add(GameStartState)
      .add(GameActiveState)
      .add(GameLostState)
      .add(MainMenuState)
  }
}

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Copy, Clone)]
pub enum GameStates {
  #[default]
  GameStart,
  GameActive,
  GameLost,
  MainMenu,
}
