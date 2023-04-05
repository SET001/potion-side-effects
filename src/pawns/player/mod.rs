use bevy::prelude::*;

use self::spawn::{spawn_player, PlayerSpawnEvent};
pub mod controller;
pub mod spawn;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<PlayerSpawnEvent>().add_system(spawn_player);
  }
}
