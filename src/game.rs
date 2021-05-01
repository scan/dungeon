use bevy::prelude::*;

use crate::{tilemap::GeneratedMap, GameState};

pub struct GamePlugin;
impl Plugin for GamePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system_set(
      SystemSet::on_enter(GameState::Running).with_system(setup.system()),
    );
  }
}

fn setup(mut commands: Commands, generated_map: Res<GeneratedMap>) {
  commands
    .spawn()
    .insert_bundle(OrthographicCameraBundle::new_2d());
}
