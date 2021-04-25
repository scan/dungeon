use anyhow::Result;
use bevy::prelude::*;
use bevy_tilemap::prelude::*;
use rand::Rng;

const TILEMAP_WIDTH: u32 = 40;
const TILEMAP_HEIGHT: u32 = 40;

pub fn generate_map(tileset_handle: Handle<TextureAtlas>) -> Result<Tilemap> {
  let mut tilemap = Tilemap::builder()
    .dimensions(TILEMAP_WIDTH, TILEMAP_HEIGHT)
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
    .texture_atlas(tileset_handle)
    .finish()?;

  let mut rng = rand::thread_rng();
  let mut tiles = Vec::new();

  for y in 0..(TILEMAP_HEIGHT as i32) {
    for x in 0..(TILEMAP_WIDTH as i32) {
      let y = y - (TILEMAP_HEIGHT as i32) / 2;
      let x = x - (TILEMAP_WIDTH as i32) / 2;

      let tile = Tile {
        point: (x, y),
        sprite_index: rng.gen_range(1..=8),
        ..Default::default()
      };
      tiles.push(tile);
    }
  }

  tilemap.insert_tiles(tiles)?;

  return Ok(tilemap);
}
