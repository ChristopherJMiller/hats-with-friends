use std::ops::Deref;

use bevy::prelude::Component;

#[derive(Component)]
pub struct PhysicsHandle<T> {
  handle: T,
}

impl<T> PhysicsHandle<T> {
  pub fn new(handle: T) -> Self {
    Self { handle }
  }
}

impl<T> Deref for PhysicsHandle<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.handle
  }
}
