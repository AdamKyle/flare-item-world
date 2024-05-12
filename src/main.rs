extern crate console_error_panic_hook;
use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;
use std::panic;

mod visibility_system;
use visibility_system::*;

mod monster_system;
pub use monster_system::*;

mod state;
pub use state::*;

mod components;
pub use components::*;

mod map;
pub use map::*;

mod rect;
pub use rect::Rect;

mod player;
pub use player::*;

/// Implements the GameState trait for the State Struct
///
/// Listens for player input
/// Draws the map
/// Sets position of entities.
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
            }
        }
    }
}

fn main() -> rltk::BError {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Tlessa Item World Demo")
        .build()?;

    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Credence".to_string(),
        })
        .build();

    gs.ecs.insert(Point::new(player_x, player_y));

    // Render Monsters
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
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

    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}
