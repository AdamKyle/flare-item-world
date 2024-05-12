use super::{Position, Renderable, Map, MonsterAI, VisibilitySystem, player_input, draw_map};
use rltk::{GameState, Rltk};
use specs::prelude::*;

/// Create a state object that holds a ecs world
pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

/// Determine the game state: Running or waiting for input
#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

/// State Logic
impl State {

    /// Run predefined game systems.
    pub fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);

        self.ecs.maintain();
    }

    /// Deterimine the state of the game
    pub fn determine_run_state(&mut self, ctx: &mut Rltk) {
        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }
    }
}

/// Implements the GameState trait for the State Struct
///
/// Listens for player input
/// Draws the map
/// Sets position of entities.
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.determine_run_state(ctx);

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
