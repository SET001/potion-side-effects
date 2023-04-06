use bevy::prelude::*;

use self::spawn::{spawn_potion, PotionSpawnEvent};
pub mod spawn;

pub enum PotionType {
  Red,
  Green,
  Blue,
}
pub struct PotionPlugin;
impl Plugin for PotionPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<PotionSpawnEvent>().add_system(spawn_potion);
  }
}
