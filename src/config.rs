use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct GameConfig {
  pub title: String,
  pub tile_size: UVec2,
  pub map_size: UVec2,
  pub gravity: f32,
  pub move_speed: f32,
  pub scale: f32,
  pub jump_initial_velocity: f32,
  pub jump_height: f32,
  pub jump_height_reach_time: f32,
  pub player_size: Vec2,
}

impl Default for GameConfig {
  fn default() -> GameConfig {
    GameConfig {
      title: "Bevy potions side effect".into(),
      tile_size: UVec2::new(16, 16),
      map_size: UVec2::new(25, 25),
      gravity: 0.,     //  auto calculated
      jump_height: 3., //  relative to player height
      jump_height_reach_time: 500.,
      jump_initial_velocity: 0., //  auto calculated
      move_speed: 200.,
      scale: 2.,
      player_size: Vec2::new(16., 20.),
    }
  }
}
