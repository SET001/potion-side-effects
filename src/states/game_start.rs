use bevy::{prelude::*, transform};
use bevy_ecs_tilemap::{
  prelude::{
    get_tilemap_center_transform, TilemapGridSize, TilemapId, TilemapSize, TilemapTexture,
    TilemapTileSize, TilemapType,
  },
  tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
  TilemapBundle,
};
use bevy_rapier2d::prelude::*;

use crate::{
  config::GameConfig,
  core::{layers::GameLayer, LevelTilemapMarker},
  pawns::{
    player::spawn::PlayerSpawnEvent,
    potion::{spawn::PotionSpawnEvent, PotionType},
  },
};

use super::GameStates;
const THIS_STATE: GameStates = GameStates::GameStart;

pub struct GameStartState;

impl Plugin for GameStartState {
  fn build(&self, app: &mut App) {
    app.add_systems(
      (background, level, on_enter, start_level)
        .chain()
        .in_schedule(OnEnter(THIS_STATE)),
    );
  }
}

fn on_enter(
  mut commands: Commands,
  mut ew_spawn_player: EventWriter<PlayerSpawnEvent>,
  mut ew_potion_spawn: EventWriter<PotionSpawnEvent>,
) {
  let root = commands
    .spawn((Name::new("Game Level"), SpatialBundle::default()))
    .id();
  ew_spawn_player.send(PlayerSpawnEvent { root });

  ew_potion_spawn.send(PotionSpawnEvent {
    potion_type: PotionType::Red,
    tile_pos: UVec2::new(20, 11),
  });

  ew_potion_spawn.send(PotionSpawnEvent {
    potion_type: PotionType::Blue,
    tile_pos: UVec2::new(7, 16),
  });

  ew_potion_spawn.send(PotionSpawnEvent {
    potion_type: PotionType::Green,
    tile_pos: UVec2::new(1, 1),
  });
}

fn level(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<GameConfig>) {
  let texture_handle: Handle<Image> = asset_server.load("tiles.png");
  let map_size = TilemapSize::from(config.map_size);
  let tilemap_entity = commands.spawn_empty().id();
  let mut tile_storage = TileStorage::empty(map_size);
  let tile_size = TilemapTileSize::from(config.tile_size.as_vec2());
  let grid_size: TilemapGridSize = tile_size.clone().into();
  let map_type = TilemapType::default();
  let mut map_transform = get_tilemap_center_transform(
    &map_size,
    &grid_size,
    &map_type,
    GameLayer::Level as i32 as f32,
  );
  map_transform.scale = Vec3::splat(config.scale);
  map_transform.translation *= config.scale;

  let mut spawn_platform_block = |x: u32, y: u32| {
    let tile_pos = TilePos { x, y };
    let tile_entity = commands
      .spawn((
        TileBundle {
          position: tile_pos,
          texture_index: TileTextureIndex(19),
          tilemap_id: TilemapId(tilemap_entity),
          ..Default::default()
        },
        Transform::from_translation(
          map_transform * tile_pos.center_in_world(&grid_size, &map_type).extend(100.),
        ),
        GlobalTransform::default(),
        RigidBody::Fixed,
        Collider::cuboid(8. * config.scale, 8. * config.scale),
      ))
      .id();
    tile_storage.set(&tile_pos, tile_entity);
  };

  for x in 0..map_size.x {
    spawn_platform_block(x, 0);
    spawn_platform_block(x, map_size.y - 1);
    spawn_platform_block(0, x);
    spawn_platform_block(map_size.x - 1, x);
  }

  for x in 0..10 {
    spawn_platform_block(x + 2, 5);
  }

  for x in 0..10 {
    spawn_platform_block(x + 12, 10);
  }

  for x in 0..15 {
    spawn_platform_block(x + 4, 15);
  }

  commands.entity(tilemap_entity).insert((
    LevelTilemapMarker,
    TilemapBundle {
      grid_size,
      map_type,
      size: map_size,
      storage: tile_storage,
      texture: TilemapTexture::Single(texture_handle),
      tile_size,
      transform: map_transform,
      ..Default::default()
    },
  ));
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

  let mut transform = get_tilemap_center_transform(
    &map_size,
    &grid_size,
    &map_type,
    GameLayer::Background as i32 as f32,
  );
  transform.scale = Vec3::splat(config.scale);
  transform.translation *= config.scale;
  commands.entity(tilemap_entity).insert(TilemapBundle {
    grid_size,
    map_type,
    size: map_size,
    storage: tile_storage,
    texture: TilemapTexture::Single(texture_handle),
    tile_size,
    transform,
    ..Default::default()
  });
}

fn start_level(mut next_state: ResMut<NextState<GameStates>>) {
  info!("Starting game...");
  *next_state = NextState(Some(GameStates::GameActive));
}
