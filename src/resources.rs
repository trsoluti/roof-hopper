//! Container for all game resources
//!
//! Resources are components that are:
//! a) Single (i.e. one per game) and
//! b) Global to the game

use amethyst::prelude::World;
use amethyst::assets::ProgressCounter;
use amethyst::ecs::Entity;
use crate::entities::load_sprite;

mod main_camera_entity_resource;
mod player_entity_resource;
mod rooftop_sprite_resource;

pub use main_camera_entity_resource::MainCameraEntityResource;
pub use player_entity_resource::PlayerEntityResource;
pub use rooftop_sprite_resource::RooftopSpriteResource;

/// Inserts the resources into the ECS World.
///
/// This has to be done *before* the entities are loaded,
/// since some entities depend on these resources.
///
/// # Parameters
///
/// `world`: The ECS `World` for this application.
/// `progress`: A progress counter for keeping track of when the resources were loaded
pub fn insert_resources(world: &mut World, progress: &mut ProgressCounter) {
    let rooftop_sprite_render = load_sprite(world, "Roof North", progress);
    world.insert(RooftopSpriteResource{ rooftop_sprite_render });
}

/// Inserts a list of entity resources into the ECS World.
///
/// This has to be done *after* the entities are loaded,
/// even though they're resources.
///
/// # Parameters
///
/// `world`: The ECS `World` for this application.
/// `entity`: The `Entity` (ID) of the player entity.
pub fn insert_entity_resources(world: &mut World, player_entity: Entity, main_camera_entity: Entity) {
    world.insert(MainCameraEntityResource { main_camera_entity });
    world.insert(PlayerEntityResource{ player_entity } );
}