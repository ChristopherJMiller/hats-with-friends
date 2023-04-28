use std::time::Duration;

use naia_bevy_shared::{LinkConditionerConfig, Protocol};

use crate::{channels::ChannelsPlugin, components::ComponentsPlugin, messages::MessagesPlugin};

// Protocol Build
pub fn protocol() -> Protocol {
  Protocol::builder()
    // Config
    .tick_interval(Duration::from_millis(16))
    .link_condition(LinkConditionerConfig::new(100, 0, 0.0))
    .enable_client_authoritative_entities()
    .add_plugin(ChannelsPlugin)
    .add_plugin(MessagesPlugin)
    .add_plugin(ComponentsPlugin)
    .build()
}
