use naia_bevy_shared::{EntityProperty, Message};

#[derive(Message)]
pub struct VectorMoveCommand {
  pub entity: EntityProperty,
  pub x: f32,
  pub y: f32,
}

impl VectorMoveCommand {
  pub fn new([x, y]: [f32; 2]) -> Self {
    Self {
      entity: EntityProperty::new_empty(),
      x,
      y
    }
  }
}
