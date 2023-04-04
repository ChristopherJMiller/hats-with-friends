use bevy::ecs::prelude::Component;

#[derive(Component)]
pub struct Interp {
  interp: f32,
  pub interp_x: f32,
  pub interp_y: f32,
  pub interp_z: f32,

  last_x: f32,
  last_y: f32,
  last_z: f32,
  pub next_x: f32,
  pub next_y: f32,
  pub next_z: f32,
}

impl Interp {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    let x = x;
    let y = y;
    let z = z;
    Self {
      interp: 0.0,
      interp_x: x,
      interp_y: y,
      interp_z: z,

      last_x: x,
      last_y: y,
      last_z: z,

      next_x: x,
      next_y: y,
      next_z: z,
    }
  }

  pub(crate) fn next_position(&mut self, next_x: f32, next_y: f32, next_z: f32) {
    self.interp = 0.0;
    self.last_x = self.next_x;
    self.last_y = self.next_y;
    self.last_z = self.next_z;

    self.interp_x = self.next_x;
    self.interp_y = self.next_y;
    self.interp_z = self.next_z;

    self.next_x = next_x;
    self.next_y = next_y;
    self.next_z = next_z;
  }

  pub(crate) fn interpolate(&mut self, interpolation: f32) {
    if self.interp >= 1.0 || interpolation == 0.0 {
      return;
    }
    if self.interp < interpolation {
      self.interp = interpolation;
      self.interp_x = self.last_x + (self.next_x - self.last_x) * self.interp;
      self.interp_y = self.last_y + (self.next_y - self.last_y) * self.interp;
      self.interp_z = self.last_z + (self.next_z - self.last_z) * self.interp;
    }
  }
}
