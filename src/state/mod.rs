use crate::prelude::*;

/// The game state.
#[derive(Clone, Debug)]
pub struct State {
  /// The visible map.
  map: Map,
  /// The player.
  player: Player,
  /// The camera.
  camera: Camera,
}

impl State {
  /// Create a new game state.
  fn new() -> Self {
    let mut rng = RandomNumberGenerator::new();
    let map_builder = MapBuilder::new(&mut rng);
    Self {
      map: map_builder.map,
      player: Player::new(map_builder.player_start),
      camera: Camera::new(map_builder.player_start),
    }
  }
}

/// Implement the GameState trait for the game state.
impl GameState for State {
  /// Update the game state.
  fn tick(&mut self, ctx: &mut BTerm) {
    ctx.set_active_console(0);
    ctx.cls();
    ctx.set_active_console(1);
    ctx.cls();
    self.player.update(ctx, &self.map, &mut self.camera);
    self.map.render(ctx, &self.camera);
    self.player.render(ctx, &self.camera);
  }
}

/// Default constructor.
impl Default for State {
  fn default() -> Self {
    Self::new()
  }
}
