// A tile type is a type of tile that can be placed on the map.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Type {
  #[default]
  Floor,
  Wall,
}
