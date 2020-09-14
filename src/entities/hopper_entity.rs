//! The hopper (player) entity
//!
//! This entity is loaded and integrated with the physics system.

use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::assets::ProgressCounter;
use amethyst::ecs::Entity;
use amethyst::core::Transform;
use crate::entities::load_sprite;
use amethyst::window::ScreenDimensions;
use crate::components::{HopperComponent, CollisionComponent};
use amethyst_rhusics::rhusics_ecs::WithPhysics;
use amethyst_rhusics::rhusics_core::collide2d::{CollisionShape2, BodyPose2};
use amethyst_rhusics::collision::CollisionStrategy;
use amethyst_rhusics::rhusics_core::{CollisionMode, Pose, PhysicalEntity};
use amethyst_rhusics::collision::primitive::Rectangle;
use cgmath::{Basis2, Point2, One};
use amethyst_rhusics::rhusics_core::physics2d::{Velocity2, Mass2};

// These values are based on an analysis of the sprite we use as our player.
const HOPPER_COLLISION_RECTANGLE_WIDTH: f32 = 68./2.;
const HOPPER_COLLISION_RECTANGLE_HEIGHT: f32 = 171./2.;
/// Loads the hopper (player) sprite as an entity and connects it to the physics system.
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
/// The created (but not yet loaded) player entity.
pub fn load_hopper_entity(world: &mut World, progress: &mut ProgressCounter, screen_dimensions: &ScreenDimensions) -> Entity {
    let mut transform = Transform::default();
    let x_pos = screen_dimensions.width() * 0.5; // middle
    let y_pos = screen_dimensions.height() * 0.3; // lower middle
    transform.set_translation_xyz(x_pos, y_pos, 1000.); // put "behind" camera
    let sprite = load_sprite(world, "Character Cat Girl", progress);
    world.create_entity()
        .with(sprite)
        .with(HopperComponent::default())
        .with(CollisionComponent::default())
        .with(transform)
        .with_dynamic_physical_entity(
            CollisionShape2::<f32, BodyPose2<f32>, ()>::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(
                    HOPPER_COLLISION_RECTANGLE_WIDTH,
                    HOPPER_COLLISION_RECTANGLE_HEIGHT).into(),
            ),
            BodyPose2::<f32>::new(
                Point2::new(x_pos, y_pos),
                Basis2::one(),
            ),
            Velocity2::<f32>::default(),
            PhysicalEntity::default(),
            Mass2::new(1.),
        )

        .build()
}
