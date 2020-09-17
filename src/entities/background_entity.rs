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
use crate::config::GAME_CONFIGURATION;

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
    transform.set_translation_xyz(
        screen_dimensions.width() * GAME_CONFIGURATION.background_rel_x_position,
        screen_dimensions.height() * GAME_CONFIGURATION.background_rel_y_position,
        GAME_CONFIGURATION.background_z_position);
    transform.set_scale(
        Vector3::new(1., 1., 1.) * GAME_CONFIGURATION.background_scale
    );
    log::info!("Background transform set to {:?}", transform);
    let sprite = load_sprite(world, "background", progress);
    world.create_entity()
        .with(sprite)
        .with(BackgroundComponent)
        .with(transform)
        .build()
}