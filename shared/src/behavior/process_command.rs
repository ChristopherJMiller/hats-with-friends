use crate::{components::Position, messages::KeyCommand};

const SQUARE_SPEED: f32 = 0.5;

pub fn process_command(key_command: &KeyCommand, position: &mut Position) {
  if key_command.w {
    *position.z += SQUARE_SPEED;
  }
  if key_command.s {
    *position.z -= SQUARE_SPEED;
  }
  if key_command.a {
    *position.x += SQUARE_SPEED;
  }
  if key_command.d {
    *position.x -= SQUARE_SPEED;
  }
}
