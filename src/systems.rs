//! Container for all the custom systems for this game
//!
//! A diagram of the system tree is given in System Tree.pdf.
//! The system tree shows which components are updated when.

use amethyst::prelude::World;
use amethyst::core::ecs::DispatcherBuilder;

mod entity_collision_system;
mod hopper_soaring_system;

pub use entity_collision_system::EntityCollisionSystem;
pub use hopper_soaring_system::HopperSoaringSystem;

/// Adds the custom systems into the game bundle.
///
/// # Parameters
///
/// - `world`: the ECS world for this application.
/// - `builder`: the `DispatchBuilder` being used to create the system bundle.
pub fn add_systems<'a, 'b>(world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) {
    builder.add(EntityCollisionSystem::default().setup(world), "entity collision system", &[]);
    builder.add(HopperSoaringSystem::default(), "hopper soaring system", &["next frame"]);
}
