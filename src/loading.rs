use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};
use bevy_tilemap::{
  prelude::{LayerKind, TilemapBundle},
  Tilemap, TilemapLayer,
};

pub struct LoadingScreenPlugin;
impl Plugin for LoadingScreenPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<TileSpriteHandles>()
      .add_startup_system(setup.system())
      .add_system(load.system());
  }
}

#[derive(Clone, Default, Debug)]
struct TileSpriteHandles {
  handles: Vec<HandleUntyped>,
  loaded: bool,
}

fn setup(
  mut tile_sprite_handles: ResMut<TileSpriteHandles>,
  asset_server: Res<AssetServer>,
) {
  tile_sprite_handles.handles = asset_server.load_folder("tiles").unwrap();
}

fn load(
  mut commands: Commands,
  mut sprite_handles: ResMut<TileSpriteHandles>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut textures: ResMut<Assets<Texture>>,
  asset_server: Res<AssetServer>,
) {
  if sprite_handles.loaded {
    return;
  }

  let mut texture_atlas_builder = TextureAtlasBuilder::default();
  if let LoadState::Loaded = asset_server
    .get_group_load_state(sprite_handles.handles.iter().map(|h| h.id))
  {
    for handle in sprite_handles.handles.iter() {
      let texture = textures.get(handle).unwrap();
      debug!("Loaded texture: {:?}", handle);
      texture_atlas_builder.add_texture(handle.clone_weak().typed(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    let tilemap = Tilemap::builder()
      .dimensions(16 * 50, 16 * 50)
      .chunk_dimensions(16, 16, 1)
      .texture_dimensions(32, 32)
      .auto_chunk()
      .auto_spawn(2, 2)
      .add_layer(
        TilemapLayer {
          kind: LayerKind::Dense,
          ..Default::default()
        },
        0,
      )
      .texture_atlas(atlas_handle)
      .finish()
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
    commands
      .spawn()
      .insert_bundle(OrthographicCameraBundle::new_2d());
    commands
      .spawn()
      .insert_bundle(tilemap_components)
      .insert(Timer::from_seconds(0.075, true));

    sprite_handles.loaded = true;
  }
}
