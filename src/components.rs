use specs_derive::*;
use specs::prelude::*;
use rltk::RGB;

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
    pub visible_tiles : Vec<rltk::Point>,
    pub range : i32,
    pub dirty : bool
}

/// Create a Player tag component we can later attach logic to.
/// 
/// This is essentially the player, things like movement and other actions are attached to it.
#[derive(Component, Debug)]
pub struct Player {}