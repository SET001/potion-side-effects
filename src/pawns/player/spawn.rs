use bevy::prelude::*;
use leafwing_input_manager::{
  prelude::{ActionState, InputMap},
  InputManagerBundle,
};

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
  // pub health: Health,
}

pub struct PlayerSpawnEvent {
  pub entity: Entity,
}

pub fn spawn_player(mut events: EventReader<PlayerSpawnEvent>, mut commands: Commands) {
  for event in events.iter() {
    // let mut input_map = InputMap::new([(KeyCode::Left, PlayerAction::MoveLeft)]);

    // commands.spawn((
    //   InputManagerBundle::<PlayerAction> {
    //     action_state: ActionState::default(),
    //     input_map,
    //   },
    //   PlayerBundle { ..default() },
    // ));
  }
}
