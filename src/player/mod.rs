use crate::prelude::*;

/// The player!
#[derive(Clone, Debug)]
pub struct Player {
  pub position: Point,
}

impl Player {
  pub fn new(position: Point) -> Self {
    Self { position }
  }

  /// Get the coordinates of the player.
  pub fn get_coordinates(&self) -> (i32, i32) {
    self.position.to_tuple()
  }

  /// Render the player.
  pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
    ctx.set_active_console(1);
    ctx.set(
      self.position.x - camera.left_x,
      self.position.y - camera.top_y,
      WHITE,
      BLACK,
      to_cp437('@'),
    );
  }

  pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
    if let Some(key) = ctx.key {
      let delta = match key {
        VirtualKeyCode::Left => Point::new(-1, 0),
        VirtualKeyCode::Right => Point::new(1, 0),
        VirtualKeyCode::Up => Point::new(0, -1),
        VirtualKeyCode::Down => Point::new(0, 1),
        _ => Point::zero(),
      };
      let new_position = self.position + delta;
      if map.is_navigable(new_position.to_tuple()) {
        self.position = new_position;
        camera.on_player_move(new_position);
      }
    }
  }
}
