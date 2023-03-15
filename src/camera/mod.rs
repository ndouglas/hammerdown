use crate::prelude::*;

/// The camera.
///
/// The camera is responsible for determining which part of the map is visible.
#[derive(Clone, Debug, Default)]
pub struct Camera {
  pub left_x: i32,
  pub right_x: i32,
  pub top_y: i32,
  pub bottom_y: i32,
}

impl Camera {
  /// Create a new camera.
  pub fn new(player_position: Point) -> Self {
    Self {
      left_x: player_position.x - DISPLAY_WIDTH / 2,
      right_x: player_position.x + DISPLAY_WIDTH / 2,
      top_y: player_position.y - DISPLAY_HEIGHT / 2,
      bottom_y: player_position.y + DISPLAY_HEIGHT / 2,
    }
  }
  /// Update the camera.
  pub fn on_player_move(&mut self, player_position: Point) {
    self.left_x = player_position.x - DISPLAY_WIDTH / 2;
    self.right_x = player_position.x + DISPLAY_WIDTH / 2;
    self.top_y = player_position.y - DISPLAY_HEIGHT / 2;
    self.bottom_y = player_position.y + DISPLAY_HEIGHT / 2;
  }

  /// Determine whether the specified coordinates are visible.
  pub fn is_visible(&self, (x, y): (i32, i32)) -> bool {
    x >= self.left_x && x < self.right_x && y >= self.top_y && y < self.bottom_y
  }

  /// Convert the specified coordinates to screen coordinates.
  pub fn to_screen_coordinates(&self, (x, y): (i32, i32)) -> (i32, i32) {
    (x - self.left_x, y - self.top_y)
  }
}
