use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct GameConfig {
  pub title: String,
  pub tile_size: UVec2,
  pub map_size: UVec2,
  pub gravity: f32,
  pub move_speed: f32,
}

impl Default for GameConfig {
  fn default() -> GameConfig {
    GameConfig {
      title: "Bevy potions side effect".into(),
      tile_size: UVec2::new(16, 16),
      map_size: UVec2::new(40, 40),
      gravity: -4.,
      move_speed: 200.,
    }
  }
}
