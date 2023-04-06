use bevy::prelude::*;
use bevy_rapier2d::prelude::{
  Collider, KinematicCharacterController, KinematicCharacterControllerOutput, RigidBody,
};
use leafwing_input_manager::{
  prelude::{ActionState, InputMap},
  InputManagerBundle,
};

use crate::{
  config::GameConfig,
  core::{health::Health, layers::GameLayer},
};

use super::controller::PlayerAction;

#[derive(Component, Default)]
pub struct Player;
#[derive(Bundle, Default)]
pub struct PlayerBundle {
  pub spatial: SpatialBundle,
  pub player: Player,
  pub name: Name,
  pub health: Health,
}

pub struct PlayerSpawnEvent {
  pub root: Entity,
}

pub fn spawn_player(
  mut events: EventReader<PlayerSpawnEvent>,
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  config: Res<GameConfig>,
) {
  for event in events.iter() {
    let input_map = InputMap::new([
      (KeyCode::Left, PlayerAction::MoveLeft),
      (KeyCode::A, PlayerAction::MoveLeft),
      (KeyCode::Right, PlayerAction::MoveRight),
      (KeyCode::D, PlayerAction::MoveRight),
      (KeyCode::Up, PlayerAction::Jump),
      (KeyCode::W, PlayerAction::Jump),
    ]);

    let player = commands
      .spawn((
        InputManagerBundle::<PlayerAction> {
          action_state: ActionState::default(),
          input_map,
        },
        PlayerBundle {
          spatial: SpatialBundle {
            transform: Transform {
              translation: Vec3::new(0., 0., GameLayer::Player as i32 as f32),
              scale: Vec3::splat(config.scale),
              ..default()
            },
            ..default()
          },
          ..default()
        },
        Collider::cuboid(8., 16.),
        RigidBody::KinematicPositionBased,
        KinematicCharacterControllerOutput::default(), //  this should be auto added by rapier
        KinematicCharacterController::default(),
      ))
      .with_children(|parent| {
        parent.spawn(SpriteBundle {
          texture: asset_server.load("guy.png"),
          ..Default::default()
        });
      })
      .id();
    commands.entity(event.root).add_child(player);
  }
}
