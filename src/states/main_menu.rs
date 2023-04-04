use bevy::prelude::*;

use super::GameStates;
pub struct MainMenuState;

const THIS_STATE: GameStates = GameStates::MainMenu;

impl Plugin for MainMenuState {
  fn build(&self, app: &mut App) {
    app.add_system(on_enter.in_schedule(OnEnter(THIS_STATE)));
  }
}

fn on_enter() {}
