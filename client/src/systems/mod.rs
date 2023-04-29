use bevy::{
  a11y::AccessibilityPlugin, app::PluginGroupBuilder, core_pipeline::CorePipelinePlugin, input::InputPlugin,
  log::LogPlugin, pbr::PbrPlugin, prelude::*, render::RenderPlugin, scene::ScenePlugin, sprite::SpritePlugin,
  text::TextPlugin, time::TimePlugin, ui::UiPlugin, winit::WinitPlugin,
};

pub mod network;
pub mod gameplay;

mod init;
pub use init::init;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MainLoop;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Tick;

pub struct BevyPlugins;

impl PluginGroup for BevyPlugins {
  fn build(self) -> bevy::app::PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      // Bevy Plugins
      .add(LogPlugin::default())
      .add(TaskPoolPlugin::default())
      .add(TypeRegistrationPlugin::default())
      .add(FrameCountPlugin::default())
      .add(AssetPlugin::default())
      .add(TimePlugin::default())
      .add(TransformPlugin::default())
      .add(InputPlugin::default())
      .add(ScenePlugin::default())
      .add(WindowPlugin {
      primary_window: Some(Window {
          fit_canvas_to_parent: true,
          canvas: Some("#bevy".to_string()),
          ..Default::default()
        }),
        ..Default::default()
      })
      .add(AccessibilityPlugin)
      .add(WinitPlugin::default())
      .add(RenderPlugin::default())
      .add(ImagePlugin::default())
      .add(CorePipelinePlugin::default())
      .add(SpritePlugin::default())
      .add(TextPlugin::default())
      .add(UiPlugin::default())
      .add(PbrPlugin::default())
  }
}
