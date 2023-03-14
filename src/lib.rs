pub mod map;
pub mod state;

pub mod prelude {
  pub use bracket_lib::prelude::*;
  pub const SCREEN_WIDTH: i32 = 80;
  pub const SCREEN_HEIGHT: i32 = 50;
  pub use crate::map::Map;
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
