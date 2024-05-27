use rltk::RGB;
use specs::prelude::*;
use specs_derive::*;

/// Create a position component
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Create a renderable component
///
/// Consists of a glyph - icon, background and forground colors
#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

/// Details what can be seen for an entity
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

// Create a Blocks Tile Tag.
#[derive(Component, Debug)]
pub struct BlocksTile {}

/// Create a Player tag component we can later attach logic to.
#[derive(Component, Debug)]
pub struct Player {}

/// Create a Monster tag component we can lattr attach logic to.
#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}
