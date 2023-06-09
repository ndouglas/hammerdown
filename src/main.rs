use hammerdown::prelude::*;

fn main() -> BError {
  let context = BTermBuilder::new()
    .with_title("Hammerdown")
    .with_fps_cap(30.0)
    .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    .with_tile_dimensions(32, 32)
    .with_resource_path("resources/")
    .with_font("dungeonfont.png", 32, 32)
    .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
    .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
    .build()?;

  main_loop(context, State::default())
}
