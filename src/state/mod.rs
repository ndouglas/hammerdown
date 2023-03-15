use crate::prelude::*;

/// The game state.
#[derive(Clone, Debug)]
pub struct State {
  /// The visible map.
  map: Map,
  /// The player.
  player: Player,
}

impl State {
  /// Create a new game state.
  pub fn new() -> Self {
    let map = Map::new();
    let player = Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2));
    Self { map, player }
  }
}

/// Implement the GameState trait for the game state.
impl GameState for State {
  /// Update the game state.
  fn tick(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    self.player.update(ctx, &self.map);
    self.map.render(ctx);
    self.player.render(ctx);
  }
}

/// Default constructor.
impl Default for State {
  fn default() -> Self {
    Self::new()
  }
}
