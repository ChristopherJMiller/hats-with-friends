use bevy::prelude::*;
use rapier3d::prelude::*;
use bevy::render::mesh::shape;

use crate::{plugins::physics::PhysicsSet, components::PhysicsHandle};

pub fn build_shared_scene_with_rendering (
  mut commands: Commands,
  mut rigidbody_set: ResMut<PhysicsSet<RigidBodySet>>,
  mut collider_set: ResMut<PhysicsSet<ColliderSet>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let (plane, ball) = build_shared_scene(&mut commands, &mut rigidbody_set, &mut collider_set);

  commands.entity(plane).insert(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(50.0).into()),
    material: materials.add(Color::SILVER.into()),
    transform: Transform::from_xyz(0.0, -10.0, 0.0),
    ..Default::default()
  });

  commands.entity(ball).insert(PbrBundle {
    mesh: meshes.add(shape::UVSphere::default().into()),
    material: materials.add(Color::SILVER.into()),
    transform: Transform::from_xyz(0.0, 1.0, 0.0),
    ..Default::default()
  });
}

pub fn build_shared_scene_without_rendering (
  mut commands: Commands,
  mut rigidbody_set: ResMut<PhysicsSet<RigidBodySet>>,
  mut collider_set: ResMut<PhysicsSet<ColliderSet>>,
) {
  let _ = build_shared_scene(&mut commands, &mut rigidbody_set, &mut collider_set);
}

fn build_shared_scene(
  commands: &mut Commands,
  rigidbody_set: &mut PhysicsSet<RigidBodySet>,
  collider_set: &mut PhysicsSet<ColliderSet>
) -> (Entity, Entity) {
  // ground plane
  let collider = ColliderBuilder::cuboid(50.0, 0.1, 50.0)
    .translation(vector![0.0, -10.0, 0.0])
    .build();
  let handle = collider_set.insert(collider);

  let plane = commands
    .spawn(PhysicsHandle::new(handle))
    .id();

  let rigid_body = RigidBodyBuilder::dynamic()
    .translation(vector![0.0, 1.0, 0.0])
    .build();
  let collider = ColliderBuilder::ball(1.0).restitution(0.7).build();
  let ball_body_handle = rigidbody_set.insert(rigid_body);
  collider_set.insert_with_parent(collider, ball_body_handle, rigidbody_set);

  let ball = commands
    .spawn(PhysicsHandle::new(ball_body_handle))
    .id();

  return (plane, ball);
}
