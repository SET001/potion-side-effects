use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::{
  prelude::{NoUserData, RapierPhysicsPlugin},
  render::RapierDebugRenderPlugin,
};

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
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(TilemapPlugin)
    .add_startup_system(setup);

  #[cfg(feature = "debug_physics")]
  app.add_plugin(RapierDebugRenderPlugin::default());

  app
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle {
    transform: Transform::from_translation(Vec3::new(0., 0., 1000.)),
    ..default()
  });
}
