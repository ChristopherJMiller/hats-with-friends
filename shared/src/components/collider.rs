use bevy::prelude::Component;
use naia_bevy_shared::{Property, Replicate};


#[derive(Component, Replicate)]
pub struct ColliderContainer {
  pub size: Property<f32>,
}

impl ColliderContainer {
  pub fn new(value: f32) -> Self {
    Self::new_complete(value)
  }
}