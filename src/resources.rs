//! Container for all game resources
//!
//! Resources are components that are:
//! a) Single (i.e. one per game) and
//! b) Global to the game

use amethyst::prelude::World;
use amethyst::assets::ProgressCounter;
use amethyst::ecs::Entity;

mod player_entity_resource;
mod rooftop_sprite_resource;

pub use player_entity_resource::PlayerEntityResource;
pub use rooftop_sprite_resource::RooftopSpriteResource;

/// Inserts the resources into the ECS World.
///
/// # Parameters
///
/// `world`: The ECS `World` for this application.
/// `progress`: A progress counter for keeping track of when the resources were loaded
pub fn insert_resources(world: &mut World, progress: &mut ProgressCounter) {

}

/// Inserts the player entity resource into the ECS World.
///
/// This has to be done *after* the entities are loaded,
/// even though it's a resource.
///
/// # Parameters
///
/// `world`: The ECS `World` for this application.
/// `entity`: The `Entity` (ID) of the player entity.
pub fn insert_player_entity_resource(world: &mut World, player_entity: Entity) {

}