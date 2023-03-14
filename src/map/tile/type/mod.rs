use crate::prelude::*;

// A tile type is a type of tile that can be placed on the map.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Type {
  #[default]
  Floor,
  Wall,
}

impl Type {
  /// Render the tile.
  pub fn render(&self, ctx: &mut BTerm, (x, y): (i32, i32)) {
    match self {
      Self::Floor => {
        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
      },
      Self::Wall => {
        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
      },
    }
  }

  /// Determine whether the tile is navigable.
  pub fn is_navigable(&self) -> bool {
    match self {
      Self::Floor => true,
      Self::Wall => false,
    }
  }
}
