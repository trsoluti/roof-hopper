//! The main camera entity for the game

use amethyst::core::ecs::Entity;

/// The resource used to keep track
/// of the main camera entity.
///
/// It is a resource rather than a field in the state
/// to allow systems to access it more easily.
pub struct MainCameraEntityResource {
    pub main_camera_entity: Entity
}