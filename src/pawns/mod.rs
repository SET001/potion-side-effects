pub mod player;
pub mod potion;

use self::{player::PlayerPlugin, potion::PotionPlugin};
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct PawnsPlugin;

impl PluginGroup for PawnsPlugin {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      .add(PlayerPlugin)
      .add(PotionPlugin)
  }
}
