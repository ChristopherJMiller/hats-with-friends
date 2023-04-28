use bevy::a11y::AccessibilityPlugin;
use bevy::app::App;
use bevy::asset::AssetPlugin;
use bevy::core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy::core_pipeline::{clear_color::ClearColor, CorePipelinePlugin};
use bevy::ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfig, SystemSet};
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::pbr::PbrPlugin;
use bevy::render::{color::Color, texture::ImagePlugin, RenderPlugin};
use bevy::scene::ScenePlugin;
use bevy::sprite::SpritePlugin;
use bevy::text::TextPlugin;
use bevy::time::TimePlugin;
use bevy::transform::TransformPlugin;
use bevy::ui::UiPlugin;
use bevy::window::{Window, WindowPlugin};
use bevy::winit::WinitPlugin;
use bevy_rapier3d::prelude::*;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin};
use shared::protocol;
use smooth_bevy_cameras::controllers::orbit::OrbitCameraPlugin;
use smooth_bevy_cameras::LookTransformPlugin;

use crate::systems::camera::CameraPlugin;
use crate::systems::connect_status::ConnectionStatusPlugin;
use crate::systems::events::ClientEventPlugin;
use crate::systems::{init, input, sync};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MainLoop;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Tick;

// TODO Clean up this file, bevy plugin group, game logic, network logics

pub fn run() {
  App::default()
        // Bevy Plugins
        .add_plugin(LogPlugin::default())
        .add_plugin(TaskPoolPlugin::default())
        .add_plugin(TypeRegistrationPlugin::default())
        .add_plugin(FrameCountPlugin::default())
        .add_plugin(AssetPlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(TransformPlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(ScenePlugin::default())
        .add_plugin(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        })
        .add_plugin(AccessibilityPlugin)
        .add_plugin(WinitPlugin::default())
        .add_plugin(RenderPlugin::default())
        .add_plugin(ImagePlugin::default())
        .add_plugin(CorePipelinePlugin::default())
        .add_plugin(SpritePlugin::default())
        .add_plugin(TextPlugin::default())
        .add_plugin(UiPlugin::default())
        .add_plugin(PbrPlugin::default())
        // Add Naia Client Plugin
        .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol()))
        // Background Color
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(ConnectionStatusPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::new(true))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())

        .add_plugin(RapierDebugRenderPlugin::default())
        // Startup System
        .add_startup_system(init)
        // Receive Client Events
        .add_plugin(ClientEventPlugin)
        // Realtime Gameplay Loop
        .configure_set(MainLoop.after(Tick))
        .add_plugin(CameraPlugin)
        .add_systems(
            (
                input::key_input,
                sync::sync_clientside_sprites,
                sync::sync_serverside_sprites,
            )
                .chain()
                .in_set(MainLoop),
        )
        // Run App
        .run();
}
