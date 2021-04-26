use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use super::tilemap::GeneratedMap;

pub struct LoadingScreenPlugin;
impl Plugin for LoadingScreenPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<GeneratedMap>()
      .add_startup_system(setup.system());
  }
}

fn setup(
  mut commands: Commands,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  asset_server: Res<AssetServer>,
) {
  let tiles_texture: Handle<Texture> = asset_server.load("tiles/tiles0.png");
  let texture_atlas =
    TextureAtlas::from_grid(tiles_texture, Vec2::new(32.0, 32.0), 9, 1);
  let atlas_handle = texture_atlases.add(texture_atlas);

  let generated_map = GeneratedMap::new_random(50, 50, 5);

  let tilemap = generated_map.tilemap(atlas_handle).unwrap();
  let tilemap_components = TilemapBundle {
    tilemap,
    visible: Visible {
      is_visible: true,
      is_transparent: true,
    },
    transform: Default::default(),
    global_transform: Default::default(),
  };

  commands.insert_resource(generated_map);

  commands
    .spawn()
    .insert_bundle(OrthographicCameraBundle::new_2d());
  commands
    .spawn()
    .insert_bundle(tilemap_components)
    .insert(Timer::from_seconds(0.075, true));
}
