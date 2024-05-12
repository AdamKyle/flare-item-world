use super::{MonsterAI, VisibilitySystem};
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

impl State {
    pub fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
