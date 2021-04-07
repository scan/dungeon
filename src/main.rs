mod component;
mod system;

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_startup_system(setup_tilemap.system())
    .add_system(animate_sprite_system.system())
    .run();
}

fn animate_sprite_system(
  time: Res<Time>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut query: Query<(
    &mut Timer,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>,
  )>,
) {
  for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
    timer.tick(time.delta_seconds());
    if timer.finished() {
      let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
      sprite.index =
        ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
    }
  }
}

fn setup(
  commands: &mut Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture_handle = asset_server.load("sprites/hero.png");
  let texture_atlas =
    TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 4, 4);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  commands
    .spawn(Camera2dBundle::default())
    .spawn(SpriteSheetBundle {
      texture_atlas: texture_atlas_handle,
      transform: Transform::from_scale(Vec3::splat(4.0)),
      ..Default::default()
    })
    .with(Timer::from_seconds(0.1, true));
}

fn setup_tilemap(asset_server: Res<AssetServer>) {
  let texture_handle = asset_server.load("tiles/tile.png");
  if let Ok(_tilemap) = Tilemap::builder()
    .texture_atlas(texture_handle)
    .tile_dimensions(32, 32)
    .dimensions(40, 30)
    .finish()
  {}
}
