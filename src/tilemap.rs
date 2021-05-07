use anyhow::Result;
use bevy::prelude::*;
use bevy_tilemap::prelude::*;
use rand::{distributions::Standard, prelude::*};
use std::{
  cmp::{max, min},
  collections::HashMap,
};

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

#[derive(Debug, PartialEq, Copy, Clone)]
struct RoomRect(Rect<i32>);

impl RoomRect {
  pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
    RoomRect(Rect {
      left: x,
      top: y,
      right: x + w,
      bottom: y + h,
    })
  }

  // Returns true if this overlaps with other
  #[allow(clippy::suspicious_operation_groupings)]
  pub fn intersect(&self, other: &Self) -> bool {
    self.0.left <= other.0.right
      && self.0.left >= other.0.left
      && self.0.top <= other.0.bottom
      && self.0.bottom >= other.0.top
  }

  pub fn center(&self) -> (i32, i32) {
    (
      (self.0.left + self.0.right) / 2,
      (self.0.top + self.0.bottom) / 2,
    )
  }

  fn apply_to_map(&self, tiles: &mut TileData) {
    for y in self.0.top + 1..=self.0.bottom {
      for x in self.0.left + 1..=self.0.right {
        tiles.insert((x, y), TileType::Floor);
      }
    }
  }
}

type TileData = HashMap<(i32, i32), TileType>;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GeneratedMap {
  width: u32,
  height: u32,
  rooms: Vec<RoomRect>,

  tiles: TileData,
}

fn apply_horizontal_tunnel(
  tiles: &mut TileData,
  left: i32,
  right: i32,
  y: i32,
) {
  for x in min(left, right)..=max(left, right) {
    tiles.insert((x, y), TileType::Floor);
  }
}

fn apply_vertical_tunnel(tiles: &mut TileData, x: i32, top: i32, bottom: i32) {
  for y in min(top, bottom)..=max(top, bottom) {
    tiles.insert((x, y), TileType::Floor);
  }
}

impl GeneratedMap {
  pub fn new_random(width: u32, height: u32, max_rooms: usize) -> Self {
    let mut tiles = TileData::with_capacity((width * height) as usize);
    let mut rooms = Vec::with_capacity(max_rooms);

    let mut rng = rand::thread_rng();

    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    for _ in 0..max_rooms {
      let w = rng.gen_range(MIN_SIZE..=MAX_SIZE);
      let h = rng.gen_range(MIN_SIZE..=MAX_SIZE);
      let x = rng.gen_range(1..(width as i32) - w - 1);
      let y = rng.gen_range(1..(height as i32) - h - 1);
      let new_room = RoomRect::new(x, y, w, h);
      let mut ok = true;

      for other_room in rooms.iter() {
        if new_room.intersect(other_room) {
          ok = false
        }
      }

      if ok {
        new_room.apply_to_map(&mut tiles);

        if !rooms.is_empty() {
          let (new_x, new_y) = new_room.center();
          let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
          if rng.gen() {
            apply_horizontal_tunnel(&mut tiles, prev_x, new_y, prev_y);
            apply_vertical_tunnel(&mut tiles, new_x, prev_y, new_y);
          } else {
            apply_vertical_tunnel(&mut tiles, prev_x, prev_y, new_y);
            apply_horizontal_tunnel(&mut tiles, prev_x, new_x, new_y);
          }
        }

        rooms.push(new_room);
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
        },
        0,
      )
      .texture_atlas(tileset_handle)
      .finish()?;

    let mut tiles = Vec::with_capacity((self.width * self.height) as usize);

    for y in 0..(self.height as i32) {
      for x in 0..(self.width as i32) {
        let py = y - (self.height as i32 / 2);
        let px = x - (self.width as i32 / 2);

        let tile = Tile {
          point: (px, py),
          sprite_index: self.tile_for_position(x, y),
          ..Default::default()
        };
        tiles.push(tile);
      }
    }

    tilemap.insert_tiles(tiles)?;

    Ok(tilemap)
  }

  fn tile_for_position(&self, x: i32, y: i32) -> usize {
    let key = (x, y);
    match self.tiles.get(&key) {
      Some(TileType::Floor) => 7,
      _ => 0,
    }
  }

  pub fn start_position(&self) -> (i32, i32) {
    self.rooms.first().unwrap().center()
  }
}
