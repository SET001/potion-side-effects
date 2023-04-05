use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

use crate::{
  config::GameConfig,
  pawns::PawnsPlugin,
  states::{GameStates, GameStatesPlugin},
};

pub fn get_app() -> App {
  let config = GameConfig::default();
  let mut app = App::new();
  app
    .insert_resource(config.clone())
    .add_state::<GameStates>()
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: config.title,
            ..Default::default()
          }),
          ..default()
        })
        .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(GameStatesPlugin)
    .add_plugins(PawnsPlugin)
    .add_plugin(TilemapPlugin)
    .add_startup_system(setup);
  app
}

fn setup(
  mut commands: Commands,
  // mut rapier_config: ResMut<RapierConfiguration>
) {
  // rapier_config.gravity = Vec2::default();
  commands.spawn(Camera2dBundle {
    transform: Transform::from_translation(Vec3::new(0., 0., 1000.)),
    ..default()
  });
}
