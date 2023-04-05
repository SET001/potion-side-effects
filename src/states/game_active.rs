use bevy::prelude::*;
use bevy_ecs_tilemap::{
  prelude::{
    get_tilemap_center_transform, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize,
    TilemapType,
  },
  tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
  TilemapBundle,
};

use crate::config::GameConfig;

use super::GameStates;
const THIS_STATE: GameStates = GameStates::GameActive;

pub struct GameActiveState;

impl Plugin for GameActiveState {
  fn build(&self, app: &mut App) {
    app
      .add_system(background.in_schedule(OnEnter(THIS_STATE)))
      .add_system(level.in_schedule(OnEnter(THIS_STATE)));
    // .add_system(on_exit.in_schedule(OnExit(THIS_STATE)))
    // .add_system(controller.in_set(OnUpdate(THIS_STATE)));
  }
}

fn level(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<GameConfig>) {
  let texture_handle: Handle<Image> = asset_server.load("tiles.png");
  let map_size = TilemapSize::from(config.map_size);
  let tilemap_entity = commands.spawn_empty().id();
  let mut tile_storage = TileStorage::empty(map_size);
  for x in 0..map_size.x {
    let tile_pos = TilePos { x, y: 0 };
    let tile_entity = commands
      .spawn(TileBundle {
        position: tile_pos,
        texture_index: TileTextureIndex(19),
        tilemap_id: TilemapId(tilemap_entity),
        ..Default::default()
      })
      .id();
    tile_storage.set(&tile_pos, tile_entity);
  }
  let tile_size = TilemapTileSize::from(config.tile_size.as_vec2());
  let grid_size = tile_size.into();
  let map_type = TilemapType::default();

  commands.entity(tilemap_entity).insert(TilemapBundle {
    grid_size,
    map_type,
    size: map_size,
    storage: tile_storage,
    texture: TilemapTexture::Single(texture_handle),
    tile_size,
    transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 10.0),
    ..Default::default()
  });
}

fn background(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<GameConfig>) {
  let texture_handle: Handle<Image> = asset_server.load("tiles.png");
  let map_size = TilemapSize::from(config.map_size);
  let tilemap_entity = commands.spawn_empty().id();
  let mut tile_storage = TileStorage::empty(map_size);

  for x in 0..map_size.x {
    for y in 0..map_size.y {
      let tile_pos = TilePos { x, y };
      let tile_entity = commands
        .spawn(TileBundle {
          position: tile_pos,
          texture_index: TileTextureIndex(40),
          tilemap_id: TilemapId(tilemap_entity),
          ..Default::default()
        })
        .id();
      tile_storage.set(&tile_pos, tile_entity);
    }
  }

  let tile_size = TilemapTileSize::from(config.tile_size.as_vec2());
  let grid_size = tile_size.into();
  let map_type = TilemapType::default();

  commands.entity(tilemap_entity).insert(TilemapBundle {
    grid_size,
    map_type,
    size: map_size,
    storage: tile_storage,
    texture: TilemapTexture::Single(texture_handle),
    tile_size,
    transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
    ..Default::default()
  });
}
