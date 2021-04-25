mod loading;
mod tilemap;

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

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
    .add_plugins(DefaultPlugins)
    .add_plugin(TilemapPlugin)
    .add_plugin(loading::LoadingScreenPlugin)
    .run();
}
