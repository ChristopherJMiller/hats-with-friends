use std::default::Default;

use bevy::asset::Handle;
use bevy::ecs::{entity::Entity, prelude::Resource};
use bevy::prelude::StandardMaterial;
use bevy::render::mesh::Mesh;

use naia_bevy_client::CommandHistory;
use shared::messages::KeyCommand;

pub struct OwnedEntity {
  pub confirmed: Entity,
  pub predicted: Entity,
}

impl OwnedEntity {
  pub fn new(confirmed_entity: Entity, predicted_entity: Entity) -> Self {
    OwnedEntity {
      confirmed: confirmed_entity,
      predicted: predicted_entity,
    }
  }
}

#[derive(Resource)]
pub struct Global {
  pub owned_entity: Option<OwnedEntity>,
  pub queued_command: Option<KeyCommand>,
  pub command_history: CommandHistory<KeyCommand>,
  pub red: Handle<StandardMaterial>,
  pub blue: Handle<StandardMaterial>,
  pub yellow: Handle<StandardMaterial>,
  pub green: Handle<StandardMaterial>,
  pub white: Handle<StandardMaterial>,
  pub player: Handle<Mesh>,
}

impl Default for Global {
  fn default() -> Self {
    Self {
      owned_entity: None,
      queued_command: None,
      command_history: CommandHistory::default(),
      white: Handle::default(),
      red: Handle::default(),
      blue: Handle::default(),
      yellow: Handle::default(),
      green: Handle::default(),
      player: Handle::default(),
    }
  }
}
