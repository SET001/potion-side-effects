use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Health {
  pub max: i32,
  pub current: i32,
}
