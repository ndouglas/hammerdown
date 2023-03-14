use crate::prelude::*;

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
  /// The height of the map.
  pub height: usize,
}

impl List {
  /// Create a new set of tiles.
  pub fn new() -> Self {
    let width = SCREEN_WIDTH as usize;
    let height = SCREEN_HEIGHT as usize;
    let tiles = vec![TileType::default(); width * height];
    Self { tiles, width, height }
  }

  /// Get the index for a specified x and y coordinate.
  pub fn get_index(&self, (x, y): (i32, i32)) -> usize {
    y as usize * self.width + x as usize
  }

  /// Get the x and y coordinates for a specified index.
  pub fn get_coordinates(&self, index: usize) -> (i32, i32) {
    let y = (index / self.width) as i32;
    let x = (index % self.width) as i32;
    (x, y)
  }

  /// Determine whether the coordinates are in bounds.
  pub fn is_in_bounds(&self, (x, y): (i32, i32)) -> bool {
    x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
  }

  /// Determine whether the tile at the specified coordinates is navigable.
  pub fn is_navigable(&self, (x, y): (i32, i32)) -> bool {
    self.is_in_bounds((x, y)) && self.tiles[self.get_index((x, y))].is_navigable()
  }

  /// Render the tiles.
  pub fn render(&self, ctx: &mut BTerm) {
    for (index, tile) in self.tiles.iter().enumerate() {
      let (x, y) = self.get_coordinates(index);
      tile.render(ctx, (x, y));
    }
  }
}

/// Default constructor.
impl Default for List {
  fn default() -> Self {
    Self::new()
  }
}
