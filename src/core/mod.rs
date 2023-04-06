use bevy::prelude::Component;

pub mod animation;
pub mod health;
pub mod layers;

#[derive(Component)]
pub struct LevelTilemapMarker;
