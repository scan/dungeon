use anyhow::{Error, Result};
use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};

#[derive(Clone, Default, Debug)]
pub struct TileSpriteHandles {
  handles: Vec<HandleUntyped>,
  loaded: bool,
}

pub fn setup(
  mut tile_sprite_handles: ResMut<TileSpriteHandles>,
  asset_server: Res<AssetServer>,
) -> Result<()> {
  tile_sprite_handles.handles = asset_server.load_folder("tiles")?;

  Ok(())
}

pub fn load(
  mut commands: Commands,
  mut sprite_handles: ResMut<TileSpriteHandles>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut textures: ResMut<Assets<Texture>>,
  asset_server: Res<AssetServer>,
) -> Result<()> {
  if sprite_handles.loaded {
    return Ok(());
  }

  let mut texture_atlas_builder = TextureAtlasBuilder::default();
  if let LoadState::Loaded = asset_server
    .get_group_load_state(sprite_handles.handles.iter().map(|h| h.id))
  {
    for handle in sprite_handles.handles.iter() {
      let texture = textures
        .get(handle)
        .ok_or(Error::msg("texture not found"))?;
      texture_atlas_builder.add_texture(handle.clone_weak().typed(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures)?;
    let atlas_handle = texture_atlases.add(texture_atlas);
  }

  Ok(())
}
