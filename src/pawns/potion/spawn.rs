use benimator::FrameRate;
use bevy::prelude::*;

use crate::core::animation::{Animation, AnimationState};
use crate::core::layers::GameLayer;

use super::PotionType;

#[derive(Component, Default)]
struct PotionMarker;
#[derive(Default, Bundle)]
struct PotionBundle {
  marker: PotionMarker,
  name: Name,
  spatial: SpatialBundle,
}
pub struct PotionSpawnEvent {
  pub potion_type: PotionType,
  pub translation: Vec2,
}

pub fn spawn_potion(
  mut commands: Commands,
  mut er_potion_spawn: EventReader<PotionSpawnEvent>,
  asset_server: Res<AssetServer>,
  mut textures: ResMut<Assets<TextureAtlas>>,
) {
  for event in er_potion_spawn.iter() {
    let animation = Animation(benimator::Animation::from_indices(
      (15..32).chain((31..16).rev()),
      FrameRate::from_fps(20 as f64),
    ));

    commands
      .spawn(PotionBundle {
        spatial: SpatialBundle {
          transform: Transform::from_xyz(0., 0., GameLayer::Potion as i32 as f32),
          ..default()
        },
        ..default()
      })
      .with_children(|parent| {
        parent.spawn((
          Name::new("potion animation"),
          AnimationState::default(),
          SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
              asset_server.load("potion.png"),
              Vec2::new(16.0, 16.0),
              33,
              1,
              None,
              None,
            )),
            transform: Transform {
              translation: Vec3::new(-50.0, 0.0, 0.0),
              ..Default::default()
            },
            ..Default::default()
          },
          animation,
        ));
      });
  }
}
