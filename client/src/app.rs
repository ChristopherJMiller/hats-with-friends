use bevy::app::App;
use bevy::core_pipeline::clear_color::ClearColor;
use bevy::prelude::IntoSystemSetConfig;
use bevy::render::color::Color;

use crate::systems::gameplay::GameplayPlugins;
use crate::systems::init;
use crate::systems::network::NetworkPlugins;
use crate::systems::{BevyPlugins, MainLoop, Tick};

pub fn run() {
  App::default()
    .add_plugins(BevyPlugins)
    .add_plugins(NetworkPlugins)
    .configure_set(MainLoop.after(Tick))
    .add_plugins(GameplayPlugins)
    .insert_resource(ClearColor(Color::BLACK))
    .add_startup_system(init)
    .run();
}
