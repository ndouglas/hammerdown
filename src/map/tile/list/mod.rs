use crate::prelude::*;

use super::constants::*;
use super::TileType;

/// A data structure encompassing the tiles that make up the map.
///
/// The tiles are stored and strided in a single vector, in row-first order.
#[derive(Clone, Debug)]
pub struct List {
  /// The tiles that make up the map.
  pub tiles: Vec<TileType>,
  /// The width of the map.
  pub width: usize,
}

impl List {
  /// Create a new set of tiles.
  pub fn new() -> Self {
    let width = SCREEN_WIDTH as usize;
    let tiles = vec![TileType::default(); NUMBER_OF_TILES];
    Self { tiles, width }
  }

  /// Get the index for a specified x and y coordinate.
  pub fn get_index(&self, (x, y): (usize, usize)) -> usize {
    y * self.width + x
  }

  /// Get the x and y coordinates for a specified index.
  pub fn get_coordinates(&self, index: usize) -> (usize, usize) {
    let y = index / self.width;
    let x = index % self.width;
    (x, y)
  }

  /// Render the tiles.
  pub fn render(&self, ctx: &mut BTerm) {
    for (index, tile) in self.tiles.iter().enumerate() {
      let (x, y) = self.get_coordinates(index);
      tile.render(ctx, (x, y));
    }
  }
}

/// Default implementation.
impl Default for List {
  fn default() -> Self {
    Self::new()
  }
}
