use benimator::FrameRate;
use bevy::prelude::*;

use crate::core::animation::Animation;

use super::spawn::Dude;

#[derive(Default, Debug, PartialEq, Clone)]
pub enum DudeState {
  Jumping,
  Falling,
  #[default]
  Idle,
  Running,
}

pub struct DudeStateUpdateEvent {
  pub old_state: DudeState,
  pub new_state: DudeState,
  pub entity: Entity,
}

pub fn update_player_state(
  mut er_update: EventReader<DudeStateUpdateEvent>,
  mut q_animation: Query<&mut Animation>,
  mut q_event_entity: Query<(&Children, &mut Dude)>,
) {
  for event in er_update.iter() {
    if let Ok((children, mut dude)) = q_event_entity.get_mut(event.entity) {
      dude.state = event.new_state.clone();
      for &child in children.into_iter() {
        if let Ok(mut animation) = q_animation.get_mut(child) {
          info!(
            "get dude state upate from {:?} to {:?}",
            event.old_state, event.new_state
          );
          let anim = match event.new_state {
            DudeState::Running => benimator::Animation::from_indices(
              (2..5).chain((4..0).rev()),
              FrameRate::from_fps(8 as f64),
            ),
            _ => benimator::Animation::from_indices(0..2, FrameRate::from_fps(8 as f64)),
          };
          animation.0 = anim;
        } else {
          warn!("No such entity with Animation component exist");
        }
      }
    }
  }
}
