use bevy::prelude::*;

use self::{
  spawn::{spawn_player, DudeSpawnEvent},
  states::{update_player_state, DudeStateUpdateEvent},
};
pub mod controller;
pub mod spawn;
pub mod states;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<DudeSpawnEvent>()
      .add_event::<DudeStateUpdateEvent>()
      .add_system(spawn_player)
      .add_system(update_player_state);
  }
}
