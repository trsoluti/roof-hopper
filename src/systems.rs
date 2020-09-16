//! Container for all the custom systems for this game
//!
//! A diagram of the system tree is given in System Tree.pdf.
//! The system tree shows which components are updated when.

use amethyst::prelude::World;
use amethyst::core::ecs::DispatcherBuilder;

mod entity_collision_system;
mod hopper_soaring_system;
mod hopper_collision_state_system;
mod hopper_rest_system;
mod hopper_jump_system;

pub use entity_collision_system::EntityCollisionSystem;
pub use hopper_soaring_system::HopperSoaringSystem;
pub use hopper_collision_state_system::HopperCollisionStateSystem;
pub use hopper_rest_system::HopperRestSystem;
pub use hopper_jump_system::HopperJumpSystem;

/// Adds the custom systems into the game bundle.
///
/// # Parameters
///
/// - `world`: the ECS world for this application.
/// - `builder`: the `DispatchBuilder` being used to create the system bundle.
pub fn add_systems<'a, 'b>(world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) {
    builder.add(EntityCollisionSystem::default().setup(world), "entity collision system", &[]);
    builder.add(HopperSoaringSystem::default(), "hopper soaring system", &["next frame"]);
    builder.add(HopperCollisionStateSystem::default(), "hopper collision state system", &["entity collision system", "hopper soaring system"]);
    builder.add(HopperRestSystem::default(), "hopper rest system", &["hopper collision state system"]);
    builder.add(HopperJumpSystem::default(), "hooper jump system", &["hopper collision state system"]);
}
