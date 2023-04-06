use bevy::{prelude::*, sprite::TextureAtlasSprite, time::Time};

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(benimator::State);

#[derive(Component, Deref, Debug, Clone)]
pub struct Animation(pub benimator::Animation);

pub struct AnimationEndedEvent {
  pub entity: Entity,
}

#[derive(Component)]
pub struct DespawnOnAnimationEnd;

pub fn animate(
  mut commands: Commands,
  mut ew_animation_ended: EventWriter<AnimationEndedEvent>,
  mut query: Query<(
    &mut AnimationState,
    &mut TextureAtlasSprite,
    &Animation,
    Entity,
  )>,
  time: Res<Time>,
  q_despawn_on_animation_end: Query<&DespawnOnAnimationEnd>,
) {
  let query_iter = query.iter_mut();
  for (mut state, mut texture, animation, entity) in query_iter {
    if state.is_ended() {
      if q_despawn_on_animation_end.get(entity).is_ok() {
        trace!("despawning entity {:?}", entity.index());
        commands.entity(entity).despawn_recursive();
      } else {
        ew_animation_ended.send(AnimationEndedEvent { entity });
      }
    } else {
      state.update(animation, time.delta());
      texture.index = state.frame_index();
    }
  }
}
