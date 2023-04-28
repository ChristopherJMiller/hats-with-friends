use std::collections::HashMap;

use bevy::ecs::system::Commands;
use bevy::log::info;

use naia_bevy_server::{transport::webrtc, Server};

use crate::resources::Global;

pub fn init(mut commands: Commands, mut server: Server) {
  info!("Naia Bevy Server Demo is running");

  // Naia Server initialization
  let server_addresses = webrtc::ServerAddrs::new(
    "0.0.0.0:14191".parse().expect("could not parse Signaling address/port"),
    // IP Address to listen on for UDP WebRTC data channels
    std::env::var("UDP_WEBRTC")
      .unwrap_or("0.0.0.0:14192".to_string())
      .parse()
      .expect("could not parse WebRTC data address/port"),
    // The public WebRTC IP address to advertise
    &std::env::var("WEBRTC_PUB_URL").unwrap_or("http://127.0.0.1:14192".to_string()),
  );
  let socket = webrtc::Socket::new(&server_addresses, server.socket_config());
  server.listen(socket);

  // Create a new, singular room, which will contain Users and Entities that they
  // can receive updates from
  let main_room_key = server.make_room().key();

  // Resources
  commands.insert_resource(Global {
    main_room_key,
    user_to_square_map: HashMap::new(),
    user_key_to_username: HashMap::new(),
  })
}
