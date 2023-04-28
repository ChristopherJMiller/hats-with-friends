use bevy::prelude::*;
use measurements::Frequency;
use rapier3d::prelude::*;
use std::ops::{Deref, DerefMut};

use crate::protocol::TICK_INTERVAL;

#[derive(Resource)]
pub struct PhysicsState {
  pub pipeline: PhysicsPipeline,
  pub islands: IslandManager,
  pub broad_phase: BroadPhase,
  pub narrow_phase: NarrowPhase,
  pub bodies: RigidBodySet,
  pub colliders: ColliderSet,
  pub joints: ImpulseJointSet,
  pub multibody_joints: MultibodyJointSet,
  pub ccd_solver: CCDSolver,
  pub query_pipeline: QueryPipeline,
  pub integration_parameters: IntegrationParameters,
  pub gravity: Vector<f32>,
}

impl PhysicsState {
  fn new(steps: f32) -> Self {
    Self {
      pipeline: Default::default(),
      islands: Default::default(),
      broad_phase: Default::default(),
      narrow_phase: Default::default(),
      bodies: Default::default(),
      colliders: Default::default(),
      joints: Default::default(),
      multibody_joints: Default::default(),
      ccd_solver: Default::default(),
      query_pipeline: Default::default(),
      integration_parameters: IntegrationParameters {
        dt: 1.0 / steps,
        min_ccd_dt: 1.0 / steps / 100.0,
        ..Default::default()
      },
      gravity: vector![0.0, -9.81, 0.0],
    }
  }
}

#[derive(Resource, Default)]
pub struct PhysicsSet<T: Default> {
  set: T,
}

impl<T: Default> Deref for PhysicsSet<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.set
  }
}

impl<T: Default> DerefMut for PhysicsSet<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.set   
  }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
  fn build(&self, app: &mut App) {
    let freq = Frequency::from_period(TICK_INTERVAL);

    app
      .insert_resource(PhysicsState::new(freq.as_hertz() as f32))
      .init_resource::<PhysicsSet<ColliderSet>>()
      .init_resource::<PhysicsSet<RigidBodySet>>();
  }
}
