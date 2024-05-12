use rltk::RGB;
use specs::prelude::*;
use super::{State, Rect, Position, Renderable, Viewshed, Monster, Name};

pub fn monster_generation(mut gs: State, rooms: Vec<Rect>) -> State {
    let mut rng = rltk::RandomNumberGenerator::new();

    for (i, room) in rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: rltk::FontCharType;
        let roll = rng.roll_dice(1, 2);
        let name: String;

        match roll {
            1 => {
                glyph = rltk::to_cp437('g');
                name = "Goblin".to_string()
            }
            _ => {
                glyph = rltk::to_cp437('o');
                name = "Orc".to_string()
            }
        }

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .build();
    }

    gs
}