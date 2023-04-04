use crate::{components::FollowPlayer, resources::Global};
use bevy::prelude::*;

pub fn camera_follow_player(
  global: Res<Global>,
  position_query: Query<&Transform, Without<FollowPlayer>>,
  mut cameras: Query<&mut Transform, With<FollowPlayer>>,
) {
  if let Some(player) = &global.owned_entity {
    if let Ok(pos) = position_query.get(player.predicted) {
      cameras.for_each_mut(|mut trans| trans.look_at(pos.translation, Vec3::Y));
    }
  }
}
