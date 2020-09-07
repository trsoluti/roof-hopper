//! Container for all the custom systems for this game

use amethyst::prelude::World;
use amethyst::core::ecs::DispatcherBuilder;

/// Adds the custom systems into the game bundle.
///
/// # Parameters
///
/// - `world`: the ECS world for this application.
/// - `builder`: the `DispatchBuilder` being used to create the system bundle.
pub fn add_systems<'a, 'b>(world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) {

}
