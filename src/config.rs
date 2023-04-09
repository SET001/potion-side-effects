use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct GameConfig {
  pub title: String,
  pub tile_size: UVec2,
  pub map_size: UVec2,
  pub gravity: f32,
  pub move_speed: f32,
  pub scale: f32,
  pub jump_height: f32,
  pub jump_height_reach_time: f32,
}

impl Default for GameConfig {
  fn default() -> GameConfig {
    GameConfig {
      title: "Bevy potions side effect".into(),
      tile_size: UVec2::new(16, 16),
      map_size: UVec2::new(25, 25),
      gravity: -10.,
      jump_height: 16. * 3.,
      jump_height_reach_time: 2000.,
      move_speed: 200.,
      scale: 2.,
    }
  }
}
