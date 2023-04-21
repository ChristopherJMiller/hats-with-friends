use crate::{
  app::MainLoop,
  components::{FollowPlayer, Player},
  resources::Global,
};
use bevy::{
  input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
  prelude::*,
};
use smooth_bevy_cameras::controllers::orbit::{ControlEvent, OrbitCameraController};

pub fn camera_input_map(
  mut events: EventWriter<ControlEvent>,
  mut mouse_wheel_reader: EventReader<MouseWheel>,
  mut mouse_motion_events: EventReader<MouseMotion>,
  mouse_buttons: Res<Input<MouseButton>>,
  controllers: Query<&OrbitCameraController>,
) {
  // Can only control one camera at a time.
  let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
    controller
  } else {
    return;
  };
  let OrbitCameraController {
    mouse_rotate_sensitivity,
    mouse_wheel_zoom_sensitivity,
    pixels_per_line,
    ..
  } = *controller;

  let mut cursor_delta = Vec2::ZERO;
  for event in mouse_motion_events.iter() {
    cursor_delta += event.delta;
  }

  if mouse_buttons.pressed(MouseButton::Right) {
    events.send(ControlEvent::Orbit(mouse_rotate_sensitivity * cursor_delta));
  }

  let mut scalar = 1.0;
  for event in mouse_wheel_reader.iter() {
    // scale the event magnitude per pixel or per line
    let scroll_amount = match event.unit {
      MouseScrollUnit::Line => event.y,
      MouseScrollUnit::Pixel => event.y / pixels_per_line,
    };
    scalar *= 1.0 - scroll_amount * mouse_wheel_zoom_sensitivity;
  }
  events.send(ControlEvent::Zoom(scalar));
}

pub fn camera_follow_player(
  global: Res<Global>,
  mut commands: Commands,
  player_query: Query<Entity, (Without<FollowPlayer>, With<Player>)>,
  mut cameras: Query<Entity, (With<FollowPlayer>, Without<Player>, Without<Parent>)>,
) {
  if let Some(player) = &global.owned_entity {
    if let Ok(player) = player_query.get(player.predicted) {
      cameras.for_each_mut(|camera_ent| {
        commands.entity(player).push_children(&[camera_ent]);
      });
    }
  }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems((camera_follow_player, camera_input_map).in_set(MainLoop));
  }
}
