use bevy::log::info;
use bevy::transform::components::Transform;
use bevy::{
  ecs::{
    event::EventReader,
    system::{Commands, Query, Res, ResMut},
  },
  prelude::{Handle, Mesh, PbrBundle},
};

use naia_bevy_client::{
  events::{
    ClientTickEvent, ConnectEvent, DespawnEntityEvent, DisconnectEvent, InsertComponentEvents, MessageEvents,
    RejectEvent, RemoveComponentEvents, SpawnEntityEvent, UpdateComponentEvents,
  },
  sequence_greater_than, Client, CommandsExt, Replicate, Tick,
};

use shared::{
  behavior as shared_behavior,
  channels::{EntityAssignmentChannel, PlayerCommandChannel},
  components::{Color, ColorValue, Position, Shape, ShapeValue},
  messages::{EntityAssignment, VectorMoveCommand},
};

use crate::components::Player;
use crate::{
  components::{Confirmed, Interp, Predicted},
  resources::{Global, OwnedEntity},
};

use super::connect_status::ConnectionStatus;

pub fn connect_events(
  client: Client,
  mut event_reader: EventReader<ConnectEvent>,
  mut status: ResMut<ConnectionStatus>,
) {
  for _ in event_reader.iter() {
    let Ok(server_address) = client.server_address() else {
            panic!("Shouldn't happen");
        };
    *status = ConnectionStatus::LoadingServer;
    info!("Client connected to: {}", server_address);
  }
}

pub fn reject_events(mut event_reader: EventReader<RejectEvent>, mut status: ResMut<ConnectionStatus>) {
  for _ in event_reader.iter() {
    info!("Client rejected from connecting to Server");
    *status = ConnectionStatus::Rejected;
  }
}

pub fn disconnect_events(mut event_reader: EventReader<DisconnectEvent>) {
  for _ in event_reader.iter() {
    info!("Client disconnected from Server");
  }
}

pub fn message_events(
  mut commands: Commands,
  client: Client,
  mut global: ResMut<Global>,
  mut event_reader: EventReader<MessageEvents>,
  position_query: Query<&Position>,
  mut status: ResMut<ConnectionStatus>,
) {
  for events in event_reader.iter() {
    for message in events.read::<EntityAssignmentChannel, EntityAssignment>() {
      let assign = message.assign;

      let entity = message.entity.get(&client).unwrap();
      if assign {
        *status = ConnectionStatus::Connected;

        info!("gave ownership of entity");

        // Here we create a local copy of the Player entity, to use for client-side prediction
        if let Ok(position) = position_query.get(entity) {
          let prediction_entity = commands
                        .entity(entity)
                        .remove::<Handle<Mesh>>()
                        .duplicate() // copies all Replicate components as well
                        .insert(PbrBundle {
                          mesh: global.player.clone(),
                          material: global.white.clone(),
                          transform: Transform::from_xyz(0.0, 0.0, 0.0),
                          ..Default::default()
                        })
                        // insert interpolation component
                        .insert(Interp::new(*position.x, *position.y, *position.z))
                        // mark as predicted
                        .insert(Predicted)
                        .insert(Player)
                        .id();

          global.owned_entity = Some(OwnedEntity::new(entity, prediction_entity));
        }
      } else {
        let mut disowned: bool = false;
        if let Some(owned_entity) = &global.owned_entity {
          if owned_entity.confirmed == entity {
            commands.entity(owned_entity.predicted).despawn();
            disowned = true;
          }
        }
        if disowned {
          info!("removed ownership of entity");
          global.owned_entity = None;
        }
      }
    }
  }
}

pub fn spawn_entity_events(mut event_reader: EventReader<SpawnEntityEvent>) {
  for SpawnEntityEvent(_entity) in event_reader.iter() {
    info!("spawned entity");
  }
}

pub fn despawn_entity_events(mut event_reader: EventReader<DespawnEntityEvent>) {
  for DespawnEntityEvent(_entity) in event_reader.iter() {
    info!("despawned entity");
  }
}

pub fn insert_component_events(
  mut commands: Commands,
  mut event_reader: EventReader<InsertComponentEvents>,
  global: Res<Global>,
  sprite_query: Query<(&Shape, &Color)>,
  position_query: Query<&Position>,
) {
  for events in event_reader.iter() {
    for entity in events.read::<Color>() {
      // When we receive a replicated Color component for a given Entity,
      // use that value to also insert a local-only SpriteBundle component into this entity
      info!("add Color Component to entity");

      if let Ok((shape, color)) = sprite_query.get(entity) {
        match *shape.value {
          // Square
          ShapeValue::Square => {
            let material = {
              match *color.value {
                ColorValue::Red => &global.red,
                ColorValue::Blue => &global.blue,
                ColorValue::Yellow => &global.yellow,
                ColorValue::Green => &global.green,
              }
              .clone()
            };

            commands
              .entity(entity)
              .insert(PbrBundle {
                mesh: global.player.clone(),
                material,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
              })
              .insert(Confirmed);
          },
        }
      }
    }
    for entity in events.read::<Position>() {
      if let Ok(position) = position_query.get(entity) {
        // initialize interpolation
        commands
          .entity(entity)
          .insert(Interp::new(*position.x, *position.y, *position.z));
      }
    }
  }
}

pub fn update_component_events(
  mut global: ResMut<Global>,
  mut event_reader: EventReader<UpdateComponentEvents>,
  mut position_query: Query<&mut Position>,
) {
  // When we receive a new Position update for the Player's Entity,
  // we must ensure the Client-side Prediction also remains in-sync
  // So we roll the Prediction back to the authoritative Server state
  // and then execute all Player Commands since that tick, using the CommandHistory helper struct
  if let Some(owned_entity) = &global.owned_entity {
    let mut latest_tick: Option<Tick> = None;
    let server_entity = owned_entity.confirmed;
    let client_entity = owned_entity.predicted;

    for events in event_reader.iter() {
      for (server_tick, updated_entity) in events.read::<Position>() {
        // If entity is owned
        if updated_entity == server_entity {
          if let Some(last_tick) = &mut latest_tick {
            if sequence_greater_than(server_tick, *last_tick) {
              *last_tick = server_tick;
            }
          } else {
            latest_tick = Some(server_tick);
          }
        }
      }
    }

    if let Some(server_tick) = latest_tick {
      if let Ok([server_position, mut client_position]) = position_query.get_many_mut([server_entity, client_entity]) {
        // Set to authoritative state
        client_position.mirror(&*server_position);

        // Replay all stored commands

        // TODO: why is it necessary to subtract 1 Tick here?
        // it's not like this in the Macroquad demo
        let modified_server_tick = server_tick.wrapping_sub(1);

        let replay_commands = global.command_history.replays(&modified_server_tick);
        for (_command_tick, command) in replay_commands {
          shared_behavior::process_command(&command, &mut client_position);
        }
      }
    }
  }
}

pub fn remove_component_events(mut event_reader: EventReader<RemoveComponentEvents>) {
  for events in event_reader.iter() {
    for (_entity, _component) in events.read::<Position>() {
      info!("removed Position component from entity");
    }
  }
}

pub fn tick_events(
  mut client: Client,
  mut global: ResMut<Global>,
  mut tick_reader: EventReader<ClientTickEvent>,
  mut position_query: Query<&mut Position>,
) {
  let Some(predicted_entity) = global
        .owned_entity
        .as_ref()
        .map(|owned_entity| owned_entity.predicted) else {
        // No owned Entity
        return;
    };

  let Some(command) = global.queued_command.take() else {
        return;
    };

  for ClientTickEvent(client_tick) in tick_reader.iter() {
    if !global.command_history.can_insert(client_tick) {
      // History is full
      continue;
    }

    // Record command
    global.command_history.insert(*client_tick, command.clone());

    // Send command
    client.send_tick_buffer_message::<PlayerCommandChannel, VectorMoveCommand>(client_tick, &command);

    if let Ok(mut position) = position_query.get_mut(predicted_entity) {
      // Apply command
      shared_behavior::process_command(&command, &mut position);
    }
  }
}
