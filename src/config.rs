use bevy::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Resource, Clone)]
pub struct GameConfig {
  pub title: String,
  pub tile_size: UVec2,
  pub map_size: UVec2,
}

impl Default for GameConfig {
  fn default() -> GameConfig {
    let mut rng = thread_rng();
    let names = ["Choko-droko", "Сoсo-Chpoko", "Fuzzy-buzzy", "Goggy-moggy"]
      .iter()
      .map(|&s| s.to_string())
      .collect::<Vec<String>>();
    GameConfig {
      title: names[rng.gen_range(0..names.len())].clone(),
      tile_size: UVec2::new(16, 16),
      map_size: UVec2::new(50, 50),
    }
  }
}
