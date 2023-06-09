pub mod camera;
pub mod map;
pub mod player;
pub mod state;

pub mod prelude {
  pub use bracket_lib::prelude::*;
  pub const SCREEN_WIDTH: i32 = 80;
  pub const SCREEN_HEIGHT: i32 = 50;
  pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
  pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
  pub use crate::camera::*;
  pub use crate::map::builder::MapBuilder;
  pub use crate::map::Map;
  pub use crate::player::Player;
  pub use crate::state::State;
}

#[cfg(test)]
pub mod test {
  use std::env::set_var;

  #[allow(unused_imports)]
  use super::*;

  pub fn init() {
    set_var("RUST_BACKTRACE", "1");
  }
}
