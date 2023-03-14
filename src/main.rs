use hammerdown::prelude::*;

fn main() -> BError {
  let context = BTermBuilder::simple80x50()
    .with_title("Hammerdown")
    .with_fps_cap(30.0)
    .build()?;
  main_loop(context, State::default())
}
