//! The background image entity
//!
//! The background image is a large picture, which
//! we load and use as an entity.

use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::assets::ProgressCounter;
use amethyst::ecs::Entity;
use amethyst::core::Transform;
use amethyst::core::math::Vector3;
use crate::entities::load_sprite;
use amethyst::window::ScreenDimensions;
use crate::components::BackgroundComponent;

/// Loads the background picture as an entity.
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
/// The created (but not yet loaded) background entity.
pub fn load_background_entity(world: &mut World, progress: &mut ProgressCounter, screen_dimensions: &ScreenDimensions) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(screen_dimensions.width() * 0., screen_dimensions.height() * 0.5, -1.);
    transform.set_scale(Vector3::new(3., 3., 3.));
    let sprite = load_sprite(world, "background", progress);
    world.create_entity()
        .with(sprite)
        .with(BackgroundComponent)
        .with(transform)
        .build()
}