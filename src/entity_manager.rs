use specs::Entity;

use super::{game_log, monster_generation, Map, Point, RunState, State};

/// Insert entities and components into the ECS.
///
/// data consists of the state - global state and the player.
/// player_position is an x,y tuple of where the player currently is.
/// map is the game's map.
///
/// We also set up the default running state
pub fn insert_entities(data: (State, Entity), player_position: (i32, i32), map: Map) -> State {
    let mut gs = data.0;
    let player_entity = data.1;

    let player_x = player_position.0;
    let player_y = player_position.1;

    gs = monster_generation(gs, map.rooms.clone());

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);

    gs.ecs.insert(game_log::GameLog {
        entries: vec!["Welcome to the item world! A place full of mystery!".to_string()],
    });

    gs
}
