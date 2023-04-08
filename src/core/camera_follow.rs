use bevy::prelude::*;

#[derive(Component, Default)]
pub struct CameraFollow {
  pub position: Vec2,
}

pub fn camera_follow(
  mut q_camera: Query<&mut Transform, With<Camera2d>>,
  q_camera_follow: Query<(&Transform, &CameraFollow), Without<Camera2d>>,
) {
  if let Ok((transform_to_follow, camera_follow)) = q_camera_follow.get_single() {
    if let Ok(mut camera_transform) = q_camera.get_single_mut() {
      camera_transform.translation =
        transform_to_follow.translation + camera_follow.position.extend(0.);
    }
  }
}
