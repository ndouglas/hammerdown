use crate::prelude::*;

pub mod builder;
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

  /// Fill the map with a tile.
  pub fn fill(&mut self, tile: TileType) {
    self.tiles.fill(tile);
  }

  /// Render the map.
  pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
    ctx.set_active_console(0);
    self.tiles.render(ctx, camera);
  }

  /// Determine whether the tile at the specified coordinates is navigable.
  pub fn is_navigable(&self, (x, y): (i32, i32)) -> bool {
    self.tiles.is_navigable((x, y))
  }
}
