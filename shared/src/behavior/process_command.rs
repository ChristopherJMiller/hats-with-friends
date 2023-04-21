use bevy::prelude::Vec2;

use crate::{components::Position, messages::VectorMoveCommand};

const SQUARE_SPEED: f32 = 0.5;

pub fn process_command(command: &VectorMoveCommand, position: &mut Position) {
  let move_vec = Vec2::new(command.x, command.y).normalize();

  if !move_vec.x.is_nan() {
    *position.x += move_vec.x * SQUARE_SPEED;
  }

  if !move_vec.y.is_nan() {
    *position.z += move_vec.y * SQUARE_SPEED;
  }
}
