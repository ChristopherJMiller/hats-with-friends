use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{connect_status::ConnectionStatusPlugin, events::ClientEventPlugin, sync::SyncPlugin};
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin};
use shared::protocol;

mod connect_status;
mod events;
mod sync;

pub struct NetworkPlugins;

impl PluginGroup for NetworkPlugins {
  fn build(self) -> bevy::app::PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      .add(ClientPlugin::new(ClientConfig::default(), protocol()))
      .add(ConnectionStatusPlugin)
      .add(ClientEventPlugin)
      .add(SyncPlugin)
  }
}
