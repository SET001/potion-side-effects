use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{prelude::ActionState, Actionlike};

use crate::{config::GameConfig, core::jump::NotGrounded};

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
    &mut Dude,
    &Children,
    Entity,
    Option<&NotGrounded>,
  )>,
  mut ew_update: EventWriter<DudeStateUpdateEvent>,
  mut q_ta: Query<&mut TextureAtlasSprite>,
  // q_jump: Query<&mut Jump>,
  config: Res<GameConfig>,
  time: Res<Time>,
) {
  if let Ok((
    mut controller,
    controller_output,
    action_state,
    player,
    children,
    player_entity,
    jump,
  )) = q_player.get_single_mut()
  {
    let mut dude_state = player.state.clone();
    let move_speed = config.move_speed * time.delta_seconds();
    let mut translation = match controller.translation {
      Some(translation) => translation,
      None => Vec2::default(),
    };
    if let Some(jump) = jump {
      translation += jump.velocity;
    } else {
      translation.y += -4.16127999;
    }

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
        commands.entity(player_entity).insert(NotGrounded {
          initial_velocity: Vec2::new(0., config.jump_initial_velocity * 10.),
          start: time.elapsed(),
          ..default()
        });
      }
      // }
      // else {
      //   if let Err(_) = q_jump.get_single() {
      //     if dude_state != DudeState::Falling {
      //       dude_state = DudeState::Falling;
      //       info!("switching player to fall state");
      //     }
      //     // sdelse {
      //     //   // info!("jump.velocity {}", jump.velocity);
      //     //   translation = jump.velocity * config.scale;
      //     // }
      //   }
    }

    // // if let Ok(jump) = q_jump.get_single() {
    // //   if (time.elapsed() - jump.start).as_millis() > 200 && controller_output.grounded {
    // //     commands.entity(player_entity).remove::<Jump>();
    // //     info!("removing jump component");
    // //   }
    // //   // sdelse {
    // //   //   // info!("jump.velocity {}", jump.velocity);
    // //   //   translation = jump.velocity * config.scale;
    // //   // }
    // // }

    // if dude_state != player.state {
    //   ew_update.send(DudeStateUpdateEvent {
    //     new_state: dude_state,
    //     old_state: player.state.clone(),
    //     entity: player_entity,
    //   });
    // }
    if translation != Vec2::default() {
      // println!("updating controller translation in controller system {translation}");
      // println!("is grounded {}", controller_output.grounded);
      controller.translation = Some(translation);
    }
  }
}
