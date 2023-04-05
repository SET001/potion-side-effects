use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{prelude::ActionState, Actionlike};

use super::spawn::Player;

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
  mut q_player: Query<
    (
      &mut KinematicCharacterController,
      &KinematicCharacterControllerOutput,
      &ActionState<PlayerAction>,
      Entity,
    ),
    With<Player>,
  >,
  mut q_jump: Query<&mut Jump>,
  time: Res<Time>,
) {
  if let Ok((mut controller, controller_output, action_state, player)) = q_player.get_single_mut() {
    let mut translation = Vec2::new(0., -4.);
    let move_speed = 200. * time.delta_seconds();
    if controller_output.grounded {
      if action_state.pressed(PlayerAction::MoveRight) {
        translation.x += move_speed;
      }
      if action_state.pressed(PlayerAction::MoveLeft) {
        translation.x -= move_speed;
      }
      if action_state.just_pressed(PlayerAction::Jump) {
        commands
          .entity(player)
          .insert(Jump(Vec2::new(translation.x, 5.)));
      }
    }

    if let Ok(mut jump) = q_jump.get_single_mut() {
      let jump_speed = 0.1;
      if jump.0.y < 0. + jump_speed {
        commands.entity(player).remove::<Jump>();
      } else {
        jump.0.y -= jump_speed;
        translation += jump.0 * 2.;
      }
    }
    controller.translation = Some(translation);
  }
}
