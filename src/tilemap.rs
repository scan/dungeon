use anyhow::Result;
use bevy::prelude::*;
use bevy_tilemap::prelude::*;
use rand::{distributions::Standard, prelude::*};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
  Wall,
  Floor,
}

impl Default for TileType {
  fn default() -> Self {
    Self::Wall
  }
}

impl Distribution<TileType> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileType {
    if rng.gen_bool(0.5) {
      TileType::Floor
    } else {
      TileType::Wall
    }
  }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GeneratedMap {
  width: u32,
  height: u32,
  rooms: usize,

  tiles: HashMap<(u32, u32), TileType>,
}

impl GeneratedMap {
  pub fn new_random(width: u32, height: u32, rooms: usize) -> Self {
    let mut tiles = HashMap::with_capacity((width * height) as usize);

    let mut rng = rand::thread_rng();
    for y in 0..width {
      for x in 0..height {
        tiles.insert((x, y), rng.gen());
      }
    }

    GeneratedMap {
      width,
      height,
      rooms,
      tiles,
    }
  }

  pub fn tilemap(
    &self,
    tileset_handle: Handle<TextureAtlas>,
  ) -> Result<Tilemap> {
    let mut tilemap = Tilemap::builder()
      .dimensions(self.width, self.height)
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

    let mut tiles = Vec::with_capacity((self.width * self.height) as usize);

    for y in 0..self.height {
      for x in 0..self.width {
        let py = (y as i32) - (self.height as i32) / 2;
        let px = (x as i32) - (self.width as i32) / 2;

        let tile = Tile {
          point: (px, py),
          sprite_index: self.tile_for_position(x, y),
          ..Default::default()
        };
        tiles.push(tile);
      }
    }

    tilemap.insert_tiles(tiles)?;

    return Ok(tilemap);
  }

  fn tile_for_position(&self, x: u32, y: u32) -> usize {
    let key = (x, y);
    match self.tiles.get(&key) {
      Some(TileType::Floor) => 7,
      _ => 0,
    }
  }
}
