use crate::prelude::*;

pub mod tile;
use tile::TileType;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

/// The visual representation of the game world.
#[derive(Clone, Debug, Default)]
pub struct Map {
  /// The tiles that make up the map.
  pub tiles: Vec<TileType>,
}

impl Map {
  /// Create a new map.
  pub fn new() -> Self {
    let tiles = vec![TileType::Floor; NUM_TILES];
    Self { tiles }
  }
}
