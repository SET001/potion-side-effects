use bevy::prelude::Component;

pub mod animation;
pub mod camera_follow;
pub mod health;
pub mod jump;
pub mod layers;

#[derive(Component)]
pub struct LevelTilemapMarker;
