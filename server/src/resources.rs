use std::collections::HashMap;

use bevy::ecs::{entity::Entity, prelude::Resource};

use naia_bevy_server::{RoomKey, UserKey};

#[derive(Resource)]
pub struct Global {
  pub main_room_key: RoomKey,
  pub user_to_square_map: HashMap<UserKey, Entity>,
  pub user_key_to_username: HashMap<UserKey, String>,
}
