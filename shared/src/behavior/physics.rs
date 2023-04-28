use bevy::prelude::info;
use rapier3d::prelude::*;
use crate::plugins::physics::PhysicsState;

pub fn step_physics(state: &mut PhysicsState, rigidbody_set: &mut RigidBodySet, collider_set: &mut ColliderSet) {
  state.pipeline.step(
    &state.gravity,
    &state.integration_parameters,
    &mut state.islands,
    &mut state.broad_phase,
    &mut state.narrow_phase,
    rigidbody_set,
    collider_set,
    &mut state.joints,
    &mut state.multibody_joints,
    &mut state.ccd_solver,
    Some(&mut state.query_pipeline),
    &(),
    &(),
  );
}
