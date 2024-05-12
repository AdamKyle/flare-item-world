extern crate console_error_panic_hook;
use specs::prelude::*;
use std::panic;

mod visibility_system;
use visibility_system::*;

mod monster_system;
pub use monster_system::*;

mod state;
pub use state::*;

mod container;
pub use container::*;

mod components;
pub use components::*;

mod map;
pub use map::*;

mod rect;
pub use rect::Rect;

mod player;
pub use player::*;

mod monster_generation;
pub use monster_generation::*;


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

    gs = Container::register_ecs_components(gs);

    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs = create_character(gs, player_x, player_y);

    gs = monster_generation(gs, map.rooms.clone());

    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}
