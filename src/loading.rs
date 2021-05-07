use bevy::{asset::LoadState, prelude::*};

use crate::{tilemap::GeneratedMap, GameState};

const TILESET_PATH: &str = "tiles/tiles0.png";
const HERO_WALK_PATH: &str = "sprites/hero.png";
const HERO_ATTACK_PATH: &str = "sprites/attack.png";

pub struct LoadingScreenPlugin;
impl Plugin for LoadingScreenPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<GeneratedMap>()
      .add_system_set(
        SystemSet::on_enter(GameState::Loading)
          .with_system(start_loading.system()),
      )
      .add_system_set(
        SystemSet::on_update(GameState::Loading)
          .with_system(check_state.system()),
      );
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadingState {
  textures: Vec<HandleUntyped>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextureAtlasAssets {
  pub tileset_atlas: Handle<TextureAtlas>,
  pub hero_walking: Handle<TextureAtlas>,
  pub hero_attacking: Handle<TextureAtlas>,
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
  let tiles_texture = asset_server.load_untyped(TILESET_PATH);
  let hero_texture = asset_server.load_untyped(HERO_WALK_PATH);
  let hero_attack_texture = asset_server.load_untyped(HERO_ATTACK_PATH);

  let textures = vec![tiles_texture, hero_texture, hero_attack_texture];
  commands.insert_resource(LoadingState { textures });
}

fn check_state(
  mut commands: Commands,
  mut state: ResMut<State<GameState>>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  asset_server: Res<AssetServer>,
  loading_state: Res<LoadingState>,
) {
  if LoadState::Loaded
    != asset_server.get_group_load_state(
      loading_state.textures.iter().map(|handle| handle.id),
    )
  {
    return;
  }

  bevy::log::debug!("Loading done");

  let tiles_texture = asset_server.get_handle(TILESET_PATH);
  let tileset_atlas =
    TextureAtlas::from_grid(tiles_texture, Vec2::new(32.0, 32.0), 9, 1);
  let tileset_atlas_handle = texture_atlases.add(tileset_atlas);

  let hero_walk_texture = asset_server.get_handle(HERO_WALK_PATH);
  let hero_walk_atlas =
    TextureAtlas::from_grid(hero_walk_texture, Vec2::new(32.0, 32.0), 4, 4);
  let hero_walk_atlas_handle = texture_atlases.add(hero_walk_atlas);

  let hero_attack_texture = asset_server.get_handle(HERO_ATTACK_PATH);
  let hero_attack_atlas =
    TextureAtlas::from_grid(hero_attack_texture, Vec2::new(32.0, 32.0), 2, 4);
  let hero_attack_atlas_handle = texture_atlases.add(hero_attack_atlas);

  commands.insert_resource(TextureAtlasAssets {
    tileset_atlas: tileset_atlas_handle,
    hero_walking: hero_walk_atlas_handle,
    hero_attacking: hero_attack_atlas_handle,
  });

  state.set(GameState::Running).unwrap();
}
