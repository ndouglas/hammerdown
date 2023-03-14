use crate::prelude::*;

/// The game state.
#[derive(Clone, Debug, Default)]
pub struct State {
  /// The visible map.
  map: Map,
}

impl State {}

/// Implement the GameState trait for the game state.
impl GameState for State {
  /// Update the game state.
  fn tick(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    self.map.render(ctx);
  }
}
