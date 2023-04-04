use bevy::prelude::Component;

use naia_bevy_shared::{Property, Replicate};

#[derive(Component, Replicate)]
pub struct Position {
  pub x: Property<f32>,
  pub y: Property<f32>,
  pub z: Property<f32>,
}

impl Position {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self::new_complete(x, y, z)
  }
}
