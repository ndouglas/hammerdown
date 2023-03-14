use crate::prelude::*;

pub mod tile;
pub use tile::TileList;
pub use tile::TileType;

/// The visual representation of the game world.
#[derive(Clone, Debug, Default)]
pub struct Map {
  /// The tiles that make up the map.
  pub tiles: TileList,
}

impl Map {
  /// Create a new map.
  pub fn new() -> Self {
    let tiles = TileList::default();
    Self { tiles }
  }

  /// Render the map.
  pub fn render(&self, ctx: &mut BTerm) {
    self.tiles.render(ctx);
  }
}
