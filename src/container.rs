use super::{State, Position, Renderable, Viewshed, Player, Monster, Name};
use specs::prelude::*;

/// A container struct providing methods for registering ECS components.
pub struct Container {}

impl Container {
    /// Registers ECS components with the given game state.
    ///
    /// # Arguments
    ///
    /// * `gs` - A mutable reference to the game state.
    ///
    /// # Returns
    ///
    /// The modified game state after registering the components.
    pub fn register_ecs_components(mut gs: State) -> State {
        gs.ecs.register::<Position>();
        gs.ecs.register::<Renderable>();
        gs.ecs.register::<Viewshed>();
        gs.ecs.register::<Player>();
        gs.ecs.register::<Monster>();
        gs.ecs.register::<Name>();
    
        gs
    }
}
