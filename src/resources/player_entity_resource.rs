//! The sprite for a player

use amethyst::core::ecs::Entity;

/// The resource used to keep track
/// of the player entity.
///
/// It is a resource rather than a field in the state
/// to allow systems to access it more easily.
#[derive(Copy, Clone, Debug)]
pub struct PlayerEntityResource {
    pub player_entity: Entity
}