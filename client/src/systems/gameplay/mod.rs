use bevy::{prelude::*, app::PluginGroupBuilder};
use bevy_rapier3d::{prelude::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin};
use smooth_bevy_cameras::{LookTransformPlugin, controllers::orbit::OrbitCameraPlugin};

use self::{camera::CameraPlugin, input::InputPlugin};

pub mod camera;
pub mod input;

pub struct GameplayPlugins;

impl PluginGroup for GameplayPlugins {
  fn build(self) -> bevy::app::PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      .add(CameraPlugin)
      .add(LookTransformPlugin)
      .add(OrbitCameraPlugin::new(true))
      .add(RapierPhysicsPlugin::<NoUserData>::default())
      .add(RapierDebugRenderPlugin::default())
      .add(InputPlugin)
  }
}
