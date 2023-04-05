use bevy::prelude::*;

use crate::pawns::player::controller::player_controller;

use super::GameStates;

const THIS_STATE: GameStates = GameStates::GameActive;

pub struct GameActiveState;

impl Plugin for GameActiveState {
  fn build(&self, app: &mut App) {
    app
      // .add_systems(on_enter.in_schedule(OnEnter(THIS_STATE)))
      .add_system(player_controller.in_set(OnUpdate(THIS_STATE)));
  }
}

fn on_enter() {}
