use bevy::asset::Assets;
use bevy::ecs::system::{Commands, ResMut};
use bevy::log::info;
use bevy::prelude::{Camera3dBundle, PbrBundle, PointLight, PointLightBundle, StandardMaterial, Transform, Vec2, Vec3};
use bevy::render::{
  color::Color,
  mesh::{shape, Mesh},
};
use naia_bevy_client::{transport::webrtc, Client};
use rand::{distributions::Alphanumeric, Rng};
use shared::messages::Auth;
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

use crate::components::FollowPlayer;
use crate::resources::{Global, SESSION_AUTH_DATA};

pub fn init(
  mut commands: Commands,
  mut client: Client,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  info!("Starting Hats with Friends");

  let auth = SESSION_AUTH_DATA
    .read()
    .expect("Failed to get Auth Singleton")
    .clone()
    .unwrap_or_else(|| {
      let user: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

      Auth::new(&user, "12345")
    });

  client.auth(auth);
  let socket = webrtc::Socket::new(
    &format!(
      "{}",
      std::option_env!("SERVER_ADDR").unwrap_or("http://127.0.0.1:14191")
    ),
    client.socket_config(),
  );
  client.connect(socket);

  // TODO Scene management should be handled by an external scene file so that server and client can share

  // Setup Camera
  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_xyz(0.0, 6., -12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
      ..Default::default()
    })
    .insert(FollowPlayer)
    .insert(OrbitCameraBundle::new(
      OrbitCameraController {
        mouse_rotate_sensitivity: Vec2::splat(0.24),
        ..Default::default()
      },
      Vec3::new(-2.0, 5.0, 5.0),
      Vec3::ZERO,
      Vec3::Y,
    ));

  // Lights
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 9000.0,
      range: 100.,
      shadows_enabled: true,
      ..Default::default()
    },
    transform: Transform::from_xyz(8.0, 16.0, 8.0),
    ..Default::default()
  });

  // ground plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(50.0).into()),
    material: materials.add(Color::SILVER.into()),
    transform: Transform::from_xyz(0.0, -10.0, 0.0),
    ..Default::default()
  });

  // Setup Global Resource
  let mut global = Global::default();

  // Load colors
  global.white = materials.add(StandardMaterial::from(Color::WHITE));
  global.red = materials.add(StandardMaterial::from(Color::RED));
  global.blue = materials.add(StandardMaterial::from(Color::BLUE));
  global.yellow = materials.add(StandardMaterial::from(Color::YELLOW));
  global.green = materials.add(StandardMaterial::from(Color::GREEN));

  // Load shapes
  global.player = meshes.add(shape::Cube::default().into());
  meshes.add(shape::Plane::default().into());

  // Insert Global Resource
  commands.insert_resource(global);
}
