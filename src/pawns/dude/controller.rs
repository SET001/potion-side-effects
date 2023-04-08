use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{prelude::ActionState, Actionlike};

use crate::config::GameConfig;

use super::{
  spawn::Dude,
  states::{DudeState, DudeStateUpdateEvent},
};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
  MoveLeft,
  MoveRight,
  Jump,
}

#[derive(Component)]
pub struct Jump(pub Vec2);

pub fn player_controller(
  mut commands: Commands,
  mut q_player: Query<(
    &mut KinematicCharacterController,
    &KinematicCharacterControllerOutput,
    &ActionState<PlayerAction>,
    &Dude,
    &Children,
    Entity,
  )>,
  mut q_jump: Query<&mut Jump>,
  mut ew_update: EventWriter<DudeStateUpdateEvent>,
  config: Res<GameConfig>,
  time: Res<Time>,
  mut q_ta: Query<&mut TextureAtlasSprite>,
) {
  if let Ok((mut controller, controller_output, action_state, player, children, player_entity)) =
    q_player.get_single_mut()
  {
    let mut translation = Vec2::new(0., config.gravity);
    let move_speed = config.move_speed * time.delta_seconds();
    let mut dude_state = DudeState::Idle;
    if action_state.pressed(PlayerAction::MoveRight) {
      for child in children.iter() {
        if let Ok(mut ta) = q_ta.get_mut(*child) {
          ta.flip_x = false;
        };
      }
      translation.x += move_speed;
      dude_state = DudeState::Running;
    }
    if action_state.pressed(PlayerAction::MoveLeft) {
      for child in children.iter() {
        if let Ok(mut ta) = q_ta.get_mut(*child) {
          ta.flip_x = true;
        };
      }
      translation.x -= move_speed;
      dude_state = DudeState::Running;
    }
    if controller_output.grounded {
      if action_state.just_pressed(PlayerAction::Jump) {
        commands
          .entity(player_entity)
          .insert(Jump(Vec2::new(translation.x, 5.)));
      }
    }
    if player.state != dude_state {
      ew_update.send(DudeStateUpdateEvent {
        new_state: dude_state,
        old_state: player.state.clone(),
        entity: player_entity,
      });
    }
    if let Ok(mut jump) = q_jump.get_single_mut() {
      let jump_speed = 0.1;
      if jump.0.y < 0. + jump_speed {
        commands.entity(player_entity).remove::<Jump>();
      } else {
        jump.0.y -= jump_speed;
        translation.y += jump.0.y * 2.;
      }
    }
    controller.translation = Some(translation);
  }
}
