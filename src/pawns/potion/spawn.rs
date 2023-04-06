use benimator::FrameRate;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::{
  get_tilemap_center_transform, TilemapGridSize, TilemapSize, TilemapTileSize, TilemapType,
};
use bevy_ecs_tilemap::tiles::TilePos;

use crate::core::animation::{Animation, AnimationState};
use crate::core::layers::GameLayer;
use crate::core::LevelTilemapMarker;

use super::PotionType;

#[derive(Component, Default)]
struct PotionMarker;
#[derive(Default, Bundle)]
struct PotionBundle {
  marker: PotionMarker,
  name: Name,
  spatial: SpatialBundle,
}
pub struct PotionSpawnEvent {
  pub potion_type: PotionType,
  pub tile_pos: UVec2,
}

pub fn spawn_potion(
  mut commands: Commands,
  mut er_potion_spawn: EventReader<PotionSpawnEvent>,
  mut textures: ResMut<Assets<TextureAtlas>>,
  q_tilemap: Query<(&TilemapGridSize, &TilemapSize, &TilemapType), With<LevelTilemapMarker>>,
  asset_server: Res<AssetServer>,
) {
  if let Ok((grid_size, map_size, map_type)) = q_tilemap.get_single() {
    let map_transform = get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0);

    for event in er_potion_spawn.iter() {
      let indices = match event.potion_type {
        PotionType::Red => (0..4).chain((4..0).rev()),
        PotionType::Green => (5..14).chain((14..5).rev()),
        PotionType::Blue => (15..32).chain((31..16).rev()),
      };

      let animation = Animation(benimator::Animation::from_indices(
        indices,
        FrameRate::from_fps(20 as f64),
      ));

      let translation = map_transform
        * TilePos::from(event.tile_pos)
          .center_in_world(&grid_size, &map_type)
          .extend(GameLayer::Potion as i32 as f32);

      trace!("spawning potion at {}:{}", event.tile_pos, translation);
      commands
        .spawn(PotionBundle {
          spatial: SpatialBundle {
            transform: Transform::from_translation(translation),
            ..default()
          },
          ..default()
        })
        .with_children(|parent| {
          parent.spawn((
            Name::new("potion animation"),
            AnimationState::default(),
            SpriteSheetBundle {
              texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("potion.png"),
                Vec2::new(16.0, 16.0),
                33,
                1,
                None,
                None,
              )),
              ..default()
            },
            animation,
          ));
        });
    }
  }
}
