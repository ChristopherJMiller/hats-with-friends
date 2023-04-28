use bevy::app::{App, ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy::core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::log::{info, LogPlugin};
use shared::plugins::physics::PhysicsPlugin;
use shared::scene::build_shared_scene_without_rendering;
use std::time::Duration;

use naia_bevy_server::{Plugin as ServerPlugin, ReceiveEvents, ServerConfig};
use shared::protocol;

mod resources;
mod systems;

use systems::{events, init};

fn main() {
  info!("Hats with Friends Server is starting up");

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
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::new(ServerConfig::default(), protocol()))
        .add_plugin(PhysicsPlugin)
        .add_startup_system(build_shared_scene_without_rendering)
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
