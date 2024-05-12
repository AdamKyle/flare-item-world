
use rltk::{Point, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use super::{Map, Player, Position, State, TileType, Viewshed, Name, Renderable, RunState};
use std::cmp::{max, min};

pub fn create_character(mut gs: State, player_x: i32, player_y: i32) -> State {

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

    gs
}

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        let mut ppos = ecs.write_resource::<Point>();

        ppos.x = pos.x;
        ppos.y = pos.y;

        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => return RunState::Paused, // Nothing happened
        Some(key) => match key {
            // Left
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            // Right
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            // Up
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            // Down
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            // Unknown
            _ => return RunState::Paused,
        },
    }

    RunState::Running
}
