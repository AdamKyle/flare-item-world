use specs::prelude::*;
use super::VisibilitySystem;

/// Create a state object that holds a ecs world
pub struct State {
    pub ecs: World
}

impl State {
    pub fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}