use bevy::prelude::*;

use crate::app::Tick;

#[derive(Resource)]
pub enum ConnectionStatus {
  BeginConnection,
  Rejected,
  LoadingServer,
  Connected,
}

impl ToString for ConnectionStatus {
  fn to_string(&self) -> String {
    match self {
      ConnectionStatus::BeginConnection => "Connecting to Game Server",
      ConnectionStatus::Rejected => "Rejected from Game Server. Wrong Password?",
      ConnectionStatus::LoadingServer => "Loading Server State",
      ConnectionStatus::Connected => "",
    }
    .to_string()
  }
}

#[derive(Component)]
struct ConnectionStatusText;

fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands
    .spawn(
      TextBundle::from_section(
        "",
        TextStyle {
          font: asset_server.load("fonts/PTMono-Regular.ttf"),
          font_size: 32.0,
          color: Color::WHITE,
        },
      )
      .with_text_alignment(TextAlignment::Left)
      .with_style(Style {
        position_type: PositionType::Absolute,
        position: UiRect {
          bottom: Val::Px(5.0),
          left: Val::Px(15.0),
          ..Default::default()
        },
        ..Default::default()
      }),
    )
    .insert(ConnectionStatusText);
}

fn update_ui(status: Res<ConnectionStatus>, mut query: Query<&mut Text, With<ConnectionStatusText>>) {
  query.for_each_mut(|mut text| {
    text.sections[0].value = status.to_string();
  });
}

pub struct ConnectionStatusPlugin;

impl Plugin for ConnectionStatusPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ConnectionStatus::BeginConnection)
      .add_startup_system(build_ui)
      .add_system(update_ui.in_set(Tick));
  }
}
