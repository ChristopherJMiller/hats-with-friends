use bevy::app::{App, ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy::core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::log::{info, LogPlugin};
use bevy::pbr::PbrPlugin;
use bevy::prelude::{ImagePlugin, AssetPlugin};
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::time::TimePlugin;
use bevy::window::{WindowPlugin, ExitCondition};
use bevy_rapier3d::prelude::*;
use std::time::Duration;

use naia_bevy_server::{Plugin as ServerPlugin, ReceiveEvents, ServerConfig};
use shared::protocol;

mod resources;
mod systems;

use systems::{events, init};

fn main() {
  info!("Hats with Friends Server starting up");

  // Build App
  App::default()
        // Plugins
        .add_plugin(TaskPoolPlugin::default())
        .add_plugin(TypeRegistrationPlugin::default())
        .add_plugin(FrameCountPlugin::default())
        .insert_resource(
            // this is needed to avoid running the server at uncapped FPS
            ScheduleRunnerSettings::run_loop(Duration::from_millis(3)),
        )
        .add_plugin(AssetPlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(WindowPlugin {
            primary_window: None,
            exit_condition: ExitCondition::DontExit,
            ..Default::default()
        })
        .add_plugin(ScenePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(RenderPlugin::default())
        .add_plugin(ImagePlugin::default())
        .add_plugin(CorePipelinePlugin::default())
        .add_plugin(PbrPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(ServerPlugin::new(ServerConfig::default(), protocol()))
        // Startup System
        .add_startup_system(init)
        // Receive Server Events
        .add_systems(
            (
                events::auth_events,
                events::connect_events,
                events::disconnect_events,
                events::error_events,
                events::tick_events,
                events::spawn_entity_events,
                events::despawn_entity_events,
                events::remove_component_events
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        // Run App
        .run();
}
