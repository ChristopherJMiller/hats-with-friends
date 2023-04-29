use bevy::prelude::*;
use naia_bevy_client::Client;
use shared::messages::VectorMoveCommand;

use crate::components::{FollowPlayer, Player};
use crate::resources::Global;
use crate::systems::MainLoop;

pub struct Controls {
  pub forward: [KeyCode; 2],
  pub backward: [KeyCode; 2],
  pub left: [KeyCode; 2],
  pub right: [KeyCode; 2],
}

impl Default for Controls {
  fn default() -> Self {
    Self {
      forward: [KeyCode::W, KeyCode::Up],
      backward: [KeyCode::S, KeyCode::Down],
      left: [KeyCode::A, KeyCode::Left],
      right: [KeyCode::D, KeyCode::Right],
    }
  }
}

fn move_vec_from_input(w: bool, a: bool, s: bool, d: bool, forward_vec: Vec2) -> [f32; 2] {
  let mut result = Vec2::ZERO;
  let right_vec = forward_vec.perp();

  if w {
    result += forward_vec;
  }
  if s {
    result -= forward_vec;
  }
  if a {
    result -= right_vec;
  }
  if d {
    result += right_vec;
  }

  result.normalize_or_zero().into()
}

fn any_keys_pressed(keyboard_input: &Input<KeyCode>, inputs: &[KeyCode]) -> bool {
  inputs.iter().find(|x| keyboard_input.pressed(**x)).is_some()
}

pub fn key_input(
  mut global: ResMut<Global>,
  client: Client,
  keyboard_input: Res<Input<KeyCode>>,
  player: Query<&Transform, (With<Player>, Without<FollowPlayer>)>,
  camera: Query<&GlobalTransform, (With<FollowPlayer>, Without<Player>)>,
) {
  if let Ok(camera) = camera.get_single() {
    if let Ok(player) = player.get_single() {
      let w = any_keys_pressed(&keyboard_input, &global.controls.forward);
      let s = any_keys_pressed(&keyboard_input, &global.controls.backward);
      let a = any_keys_pressed(&keyboard_input, &global.controls.left);
      let d = any_keys_pressed(&keyboard_input, &global.controls.right);

      let player_xz = Vec2::new(player.translation.x, player.translation.z);
      let camera_xz = Vec2::new(camera.translation().x, camera.translation().z);

      let forward_vec = player_xz - camera_xz;
      let [x, y] = move_vec_from_input(w, a, s, d, forward_vec);

      if let Some(command) = &mut global.queued_command {
        command.x = x;
        command.y = y;
      } else if let Some(owned_entity) = &global.owned_entity {
        let mut move_command = VectorMoveCommand::new([x, y]);
        move_command.entity.set(&client, &owned_entity.confirmed);
        global.queued_command = Some(move_command);
      }
    }
  }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(key_input.in_set(MainLoop));
  }
}