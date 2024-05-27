use super::{
    CombatStats, Map, Name, Player, Position, Renderable, RunState, State, TileType, Viewshed,
    WantsToMelee,
};
use rltk::{console, Point, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn create_character(mut gs: State, player_x: i32, player_y: i32) -> (State, Entity) {
    let player_entity = gs
        .ecs
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
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .build();

    gs.ecs.insert(Point::new(player_x, player_y));

    (gs, player_entity)
}

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let entities = ecs.entities();
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();

    for (entity, _player, pos, viewshed) in
        (&entities, &mut players, &mut positions, &mut viewsheds).join()
    {
        let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Add target failed");
                return;
            }
        }

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
        None => return RunState::AwaitingInput, // Nothing happened
        Some(key) => match key {
            // Left
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            // Right
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            // Up
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            // Down
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),

            // Diagonaly Left Up
            VirtualKeyCode::E => try_move_player(1, -1, &mut gs.ecs),

            // Diagonaly Right Up
            VirtualKeyCode::Q => try_move_player(-1, -1, &mut gs.ecs),

            // Diagonaly Left Down
            VirtualKeyCode::C => try_move_player(1, 1, &mut gs.ecs),

            // Diagonly Right Down
            VirtualKeyCode::Z => try_move_player(-1, 1, &mut gs.ecs),

            // Unknown
            _ => return RunState::AwaitingInput,
        },
    }

    RunState::PlayerTurn
}
