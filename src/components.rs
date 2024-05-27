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

/// Details Combat Stats for Entities
///
/// Right now this is used for both entities and players.
#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

/// When we want to attack, we want to set th target as the entity.
///
/// This can be used for any entity that wants to hit any other entity.
#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
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

/// An entity for tracking damage.
#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

/// Track damage on the entity.
///
/// Track and store the damage for an entity.
impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage {
                amount: vec![amount],
            };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}
