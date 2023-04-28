use bevy::prelude::info;
use rapier3d::prelude::*;
use crate::plugins::physics::PhysicsState;

pub fn step_physics(client_tick: Option<u16>, state: &mut PhysicsState, rigidbody_set: &mut RigidBodySet, collider_set: &mut ColliderSet) {
  let number_of_steps = if let Some(client_tick) = client_tick {
    let ticks = (client_tick - state.client_step).max(1);

    if ticks > 1 {
      info!("Catching up physics state by {} steps", ticks - 1);
    }

    ticks
  } else {
    1
  };

  (0..number_of_steps).for_each(|_| {
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

    state.client_step += 1;
  });
}
