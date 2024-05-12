extern crate console_error_panic_hook; 
use std::panic;
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod visibility_system;
use visibility_system::*;

mod state;
pub use state::State;

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
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
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
        ecs: World::new()
    };
    
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map : Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range : 8, dirty: true})
        .build();

    rltk::main_loop(context, gs)
}