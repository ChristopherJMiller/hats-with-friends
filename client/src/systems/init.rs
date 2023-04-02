use bevy::asset::Assets;
use bevy::core_pipeline::prelude::Camera2dBundle;
use bevy::ecs::system::{Commands, ResMut};
use bevy::log::info;
use bevy::render::{
  color::Color,
  mesh::{shape, Mesh},
};
use bevy::sprite::ColorMaterial;

use naia_bevy_client::{transport::webrtc, Client};
use shared::messages::Auth;

use crate::resources::Global;

pub fn init(
  mut commands: Commands,
  mut client: Client,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  info!("Naia Bevy Client Demo started");

  client.auth(Auth::new("charlie", "12345"));
  let socket = webrtc::Socket::new("http://127.0.0.1:14191", client.socket_config());
  client.connect(socket);

  // Setup Camera
  commands.spawn(Camera2dBundle::default());

  // Setup Global Resource
  let mut global = Global::default();

  // Load colors
  global.white = materials.add(ColorMaterial::from(Color::WHITE));
  global.red = materials.add(ColorMaterial::from(Color::RED));
  global.blue = materials.add(ColorMaterial::from(Color::BLUE));
  global.yellow = materials.add(ColorMaterial::from(Color::YELLOW));
  global.green = materials.add(ColorMaterial::from(Color::GREEN));

  // Load shapes
  global.circle = meshes.add(shape::Circle::new(6.).into());

  // Insert Global Resource
  commands.insert_resource(global);
}
