use bevy::prelude::*;
use leafwing_input_manager::{
  prelude::{ActionState, InputMap},
  InputManagerBundle,
};

use crate::core::layers::GameLayer;

use super::controller::PlayerAction;

#[derive(Component, Default)]
pub struct Player;
#[derive(Bundle, Default)]
pub struct PlayerBundle {
  // pub velocity: VelocityComponent,
  pub transform: Transform,
  pub global_transform: GlobalTransform,
  pub player: Player,
  pub name: Name,
  pub visibility: VisibilityBundle,
  // pub health: Health,
}

pub struct PlayerSpawnEvent {
  pub root: Entity,
}

pub fn spawn_player(
  mut events: EventReader<PlayerSpawnEvent>,
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  for event in events.iter() {
    let mut input_map = InputMap::new([(KeyCode::Left, PlayerAction::MoveLeft)]);

    let player = commands
      .spawn((
        InputManagerBundle::<PlayerAction> {
          action_state: ActionState::default(),
          input_map,
        },
        PlayerBundle {
          transform: Transform::from_xyz(0., 0., GameLayer::Player as i32 as f32),
          ..default()
        },
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
