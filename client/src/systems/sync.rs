use bevy::ecs::{query::With, system::Query};
use bevy::transform::components::Transform;

use naia_bevy_client::Client;
use shared::components::Position;

use crate::components::{Confirmed, Interp, Predicted};

pub fn sync_clientside_sprites(
  client: Client,
  mut query: Query<(&Position, &mut Interp, &mut Transform), With<Predicted>>,
) {
  for (position, mut interp, mut transform) in query.iter_mut() {
    if *position.x != interp.next_x || *position.y != interp.next_y || *position.z != interp.next_z {
      interp.next_position(*position.x, *position.y, *position.z);
    }

    let interp_amount = client.client_interpolation().unwrap();
    interp.interpolate(interp_amount);
    transform.translation.x = interp.interp_x;
    transform.translation.y = interp.interp_y;
    transform.translation.z = interp.interp_z;
  }
}

pub fn sync_serverside_sprites(
  client: Client,
  mut query: Query<(&Position, &mut Interp, &mut Transform), With<Confirmed>>,
) {
  for (position, mut interp, mut transform) in query.iter_mut() {
    if *position.x != interp.next_x || *position.y != interp.next_y || *position.z != interp.next_z {
      interp.next_position(*position.x, *position.y, *position.z);
    }

    let interp_amount = client.server_interpolation().unwrap();
    interp.interpolate(interp_amount);
    transform.translation.x = interp.interp_x;
    transform.translation.y = interp.interp_y;
    transform.translation.z = interp.interp_z;
  }
}
