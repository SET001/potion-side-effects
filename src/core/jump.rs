use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::config::GameConfig;

#[derive(Component, Default)]
pub struct NotGrounded {
  pub start: Duration,
  pub initial_velocity: Vec2,
  pub velocity: Vec2,
  // pub max_height: f32,
}

pub fn jumping(
  mut commands: Commands,
  mut q_jump: Query<(
    Option<&mut NotGrounded>,
    &mut KinematicCharacterController,
    &KinematicCharacterControllerOutput,
    Entity,
  )>,
  config: Res<GameConfig>,
  time: Res<Time>,
) {
  for (mut not_grounded, mut controller, controller_output, entity) in q_jump.iter_mut() {
    let mut translation = match controller.translation {
      Some(translation) => translation,
      None => Vec2::default(),
    };

    match not_grounded {
      Some(mut not_grounded) => {
        if controller_output.grounded {
          println!("removing not_grounded component");
          commands.entity(entity).remove::<NotGrounded>();
        } else {
          let duration = time.elapsed() - not_grounded.start;
          let velocity = (Vec2::new(0., config.gravity * (duration.as_millis() as f32))
            + not_grounded.initial_velocity)
            * 8.;
          not_grounded.velocity = velocity;
          // translation += velocity * config.scale;
          // if translation != Vec2::default() {
          //   println!("updating controller translation in jump system {translation}");
          //   controller.translation = Some(translation);
          // }
        }
      }
      None => {
        if !controller_output.grounded {
          println!(
            "adding not_grounded component: {}",
            controller_output.grounded
          );
          commands.entity(entity).insert(NotGrounded {
            start: time.elapsed(),
            ..default()
          });
        } else {
          // controller.translation = Some(Vec2::new(20., 0.));
        }
      }
    }

    // if let Some(not_grounded) = not_grounded {
    //   if controller_output.grounded {
    //     println!("removing not_grounded component");
    //     commands.entity(entity).insert(NotGrounded {
    //       start: time.elapsed(),
    //       ..default()
    //     });
    //   } else {
    //     let duration = time.elapsed() - not_grounded.start;
    //     let velocity = (Vec2::new(0., config.gravity * (duration.as_millis() as f32))
    //       + not_grounded.initial_velocity)
    //       * 8.;
    //     translation = velocity * config.scale;
    //     if translation != Vec2::default() {
    //       println!("updating controller translation in jump system");
    //       controller.translation = Some(translation);
    //     }
    //   }
    // } else {
    //   if !controller_output.grounded {
    //     commands.entity(entity).remove::<NotGrounded>();
    //     println!("adding not_grounded component");
    //   }
    // }
  }
  // for (mut jump, mut controller) in q_jump.iter_mut() {
  //   let duration = time.elapsed() - jump.start;
  //   jump.velocity = Vec2::new(
  //     0.,
  //     config.gravity * (duration.as_millis() as f32) + config.jump_initial_velocity,
  //   ) * 8.;

  //   info!(
  //     "jump duration {}, jump velocity {}",
  //     duration.as_millis(),
  //     jump.velocity
  //   );
  //   controller.translation = Some(jump.velocity * config.scale);
  // }
}
