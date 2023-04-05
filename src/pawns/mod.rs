use bevy::{app::PluginGroupBuilder, prelude::*};

use self::player::PlayerPlugin;

pub mod player;

pub struct PawnsPlugin;

impl PluginGroup for PawnsPlugin {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>().add(PlayerPlugin)
  }
}
