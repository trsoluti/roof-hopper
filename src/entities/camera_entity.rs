//! The Camera entity

use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::window::ScreenDimensions;
use amethyst::core::Transform;
use amethyst::renderer::Camera;
use amethyst::ecs::Entity;

/// Initialises the camera entity.
///
/// # Parameters
///
/// - `world`: The ECS `World` of our application.
/// - `progress`: A counter that can inform us when all the entities are loaded.
/// - `screen_dimensions`: The current dimensions of the screen, in pixels.
///         These are used to place and size the background relative to the screen.
///
/// # Returns
///
/// A standard 2D camera entity centered on the screen.
pub fn initialise_camera(world: &mut World, dimensions: &ScreenDimensions) -> Entity {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build()
}
