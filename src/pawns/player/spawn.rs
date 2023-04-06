use benimator::FrameRate;
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
  core::{
    animation::{Animation, AnimationState},
    health::Health,
    layers::GameLayer,
  },
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
  mut textures: ResMut<Assets<TextureAtlas>>,
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
        let animation = Animation(benimator::Animation::from_indices(
          (2..5).chain((4..0).rev()),
          FrameRate::from_fps(20 as f64),
        ));

        parent.spawn((
          animation,
          AnimationState::default(),
          SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
              asset_server.load("player.png"),
              Vec2::new(16.0, 20.0),
              11,
              1,
              None,
              None,
            )),
            ..default()
          },
        ));
      })
      .id();
    commands.entity(event.root).add_child(player);
  }
}
