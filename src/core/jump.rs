use std::time::Duration;

use bevy::prelude::*;

use crate::config::GameConfig;

#[derive(Component, Default)]
pub struct Jump {
  pub start: Duration,
  pub initial_velocity: Vec2,
  pub velocity: Vec2,
}

pub fn jumping(config: Res<GameConfig>, mut q_jump: Query<&mut Jump>, time: Res<Time>) {
  for mut jump in q_jump.iter_mut() {
    let duration = time.elapsed() - jump.start;
    jump.velocity =
      10. * (Vec2::new(0., config.gravity * (duration.as_millis() as f32)) + jump.initial_velocity);

    info!("jump duration {}", duration.as_millis());
  }
}
