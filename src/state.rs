use specs::prelude::*;

/// Create a state object that holds a ecs world
pub struct State {
    pub ecs: World
}

/// Implements the State struct
impl State {

    /// Runs the core system.
    pub fn run_systems(&mut self) {
        self.ecs.maintain(); // applies queued up changes now
    }
}