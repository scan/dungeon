mod actions;
mod game;
mod loading;
mod tilemap;

use bevy::prelude::*;
use bevy_tilemap::prelude::*;
use heron::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
  Loading,
  Running,
}

impl Default for GameState {
  fn default() -> Self {
    Self::Loading
  }
}

fn main() {
  App::build()
    .insert_resource(bevy::log::LogSettings {
      level: bevy::log::Level::DEBUG,
      filter: "wgpu=warn,bevy_ecs=info".to_string(),
    })
    .insert_resource(WindowDescriptor {
      title: "dungeon".to_string(),
      width: 1280.0,
      height: 720.0,
      vsync: true,
      ..Default::default()
    })
    .add_state(GameState::Loading)
    .add_plugins(DefaultPlugins)
    .add_plugin(TilemapPlugin)
    .add_plugin(PhysicsPlugin::default())
    .add_plugin(actions::ActionsPlugin)
    .add_plugin(loading::LoadingScreenPlugin)
    .add_plugin(game::GamePlugin)
    .run();
}
