use bevy::{prelude::*, render::camera::Camera};
use bevy_tilemap::prelude::*;

use crate::{
  actions::Actions, loading::TextureAtlasAssets, tilemap::GeneratedMap,
  GameState,
};

pub struct Player;

pub struct GamePlugin;
impl Plugin for GamePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system_set(
        SystemSet::on_enter(GameState::Running)
          .with_system(setup.system())
          .with_system(generate_map.system()),
      )
      .add_system_set(
        SystemSet::on_update(GameState::Running)
          .with_system(move_player.system())
          .with_system(center_camera.system()),
      );
  }
}

fn generate_map(
  mut commands: Commands,
  texture_atlas_handles: Res<TextureAtlasAssets>,
) {
  let generated_map = GeneratedMap::new_random(40, 35, 10);

  let tilemap = generated_map
    .tilemap(texture_atlas_handles.tileset_atlas.clone())
    .unwrap();
  let tilemap_components = TilemapBundle {
    tilemap,
    visible: Visible {
      is_visible: true,
      is_transparent: true,
    },
    transform: Default::default(),
    global_transform: Default::default(),
  };
  let (start_x, start_y) = generated_map.start_position();

  commands.insert_resource(generated_map);

  commands.spawn().insert_bundle(tilemap_components);
  commands
    .spawn_bundle(SpriteSheetBundle {
      texture_atlas: texture_atlas_handles.hero_walking.clone_weak(),
      transform: Transform::from_translation(Vec3::new(
        start_x as f32,
        start_y as f32,
        5.,
      )),
      ..Default::default()
    })
    .insert(Player);
}

fn setup(mut commands: Commands) {
  commands
    .spawn()
    .insert_bundle(OrthographicCameraBundle::new_2d());
}

fn move_player(
  time: Res<Time>,
  actions: Res<Actions>,
  mut player_query: Query<&mut Transform, With<Player>>,
) {
  if actions.player_movement.is_none() {
    return;
  }

  let speed = 150.;
  let movement = Vec3::new(
    actions.player_movement.unwrap().x * speed * time.delta_seconds(),
    actions.player_movement.unwrap().y * speed * time.delta_seconds(),
    0.,
  );

  for mut player_transform in player_query.iter_mut() {
    player_transform.translation += movement;
  }
}

fn center_camera(
  mut q: QuerySet<(
    Query<&Transform, (Changed<Transform>, With<Player>)>,
    Query<&mut Transform, With<Camera>>,
  )>,
) {
  let mut player_translation = None;

  for player_transform in q.q0().iter() {
    player_translation = Some(player_transform.translation);
    break;
  }

  if let Some(pos) = player_translation {
    for mut camera_transform in q.q1_mut().iter_mut() {
      camera_transform.translation = pos.clone();
    }
  }
}
