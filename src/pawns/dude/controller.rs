use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{prelude::ActionState, Actionlike};

use crate::{config::GameConfig, core::jump::Jump};

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
  mut ew_update: EventWriter<DudeStateUpdateEvent>,
  mut q_ta: Query<&mut TextureAtlasSprite>,
  q_jump: Query<&mut Jump>,
  config: Res<GameConfig>,
  time: Res<Time>,
) {
  if let Ok((mut controller, controller_output, action_state, player, children, player_entity)) =
    q_player.get_single_mut()
  {
    let mut translation = Vec2::new(0., config.gravity);
    let move_speed = config.move_speed * time.delta_seconds();
    let mut dude_state = DudeState::Idle;
    if controller_output.grounded {
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
      if action_state.just_pressed(PlayerAction::Jump) {
        info!("inserting jump component...");
        commands.entity(player_entity).insert(Jump {
          initial_velocity: Vec2::new(0., 150.),
          start: time.elapsed(),
          ..default()
        });
      }
    } else {
    }

    if let Ok(jump) = q_jump.get_single() {
      if (time.elapsed() - jump.start).as_millis() > 50 && controller_output.grounded {
        commands.entity(player_entity).remove::<Jump>();
        info!("removing jump component");
      } else {
        info!("jump.velocity {}", jump.velocity);
        translation += jump.velocity;
      }
    }

    if player.state != dude_state {
      ew_update.send(DudeStateUpdateEvent {
        new_state: dude_state,
        old_state: player.state.clone(),
        entity: player_entity,
      });
    }

    controller.translation = Some(translation);
  }
}
