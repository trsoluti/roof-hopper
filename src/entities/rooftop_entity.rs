//! The rooftop entity
//!
//! We load one rooftop, with collision on,
//! as our main, initial rooftop.
//! The other rooftops are generated at the start
//! of our Game Running State.
//! (or by a spawner)

use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::assets::ProgressCounter;
use amethyst::core::Transform;
use amethyst::ecs::Entity;
use amethyst::window::ScreenDimensions;
use amethyst_rhusics::rhusics_ecs::WithPhysics;
use amethyst_rhusics::rhusics_core::collide2d::{CollisionShape2, BodyPose2};
use amethyst_rhusics::collision::CollisionStrategy;
use amethyst_rhusics::rhusics_core::{CollisionMode, Pose, PhysicalEntity};
use amethyst_rhusics::collision::primitive::Rectangle;
use cgmath::{Point2, One};
use cgmath::Basis2;
use amethyst_rhusics::rhusics_core::physics2d::{Mass2, Velocity2};
use crate::components::RooftopComponent;
use crate::resources::RooftopSpriteResource;

// These values were created from analysis of the
// sprite. The rectangle is 101 x 171 but the visible part is
// only 101 x 81.
const ROOFTOP_COLLISION_RECTANGLE_WIDTH: f32 = 101./2.;
const ROOFTOP_COLLISION_RECTANGLE_HEIGHT: f32 = 81./2.;

/// Creates an entity using the (preloaded) system resource
/// for the rooftop sprite,
/// places it at the middle bottom of the screen
/// (i.e. below the initial position of the player),
/// and connects the entity to the collision system.
///
/// Note the rooftop system resource (`SpriteRender`) needs
/// to be created before this function is called.
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
/// The created (but not yet loaded) rooftop entity.
pub fn load_rooftop_entity(world: &mut World, _progress: &mut ProgressCounter, screen_dimensions: &ScreenDimensions) -> Entity {
    create_rooftop_entity(world, screen_dimensions, Point2::new(0.5, 0.2), false, true)
}

/// Creates an entity using the (preloaded) system resource
/// for the rooftop sprite,
/// places it at the given location,
/// and connects the entity to the collision system.
///
/// Note the rooftop system resource (`SpriteRender`) needs
/// to be created before this function is called.
///
/// # Parameters
///
/// - `world`: The ECS `World` of our application.
/// - `progress`: A counter that can inform us when all the entities are loaded.
/// - `screen_dimensions`: The current dimensions of the screen, in pixels.
///         These are used to place and size the background relative to the screen.
/// - `position`: where to place the entity on the screen (0,0) to (1,1).
/// - `visible`: true if the rooftop is immediately "visible" (i.e. z==0).
///         Normally all the loading sprites are invisible until we
///         start the game, but generated entities above the roof can be
///         loaded as 'visible'.
/// - `enabled`: true if the rooftop is created with collision enabled.
///         Only the base (first) rooftop is created with collision enabled.
///         The remaining rooftops have their collision enabled when the
///         player has risen above them for the first time.
///
/// # Returns
///
/// The created (but possibly not yet loaded) rooftop entity.
pub fn create_rooftop_entity(
    world: &mut World,
    screen_dimensions: &ScreenDimensions,
    position: Point2<f32>, // in range 0. to 1.
    visible: bool,
    enabled: bool,
) -> Entity {
    let mut transform = Transform::default();
    let x_pos = screen_dimensions.width() * position.x;
    let y_pos = screen_dimensions.height() * position.y;
    let z_pos = if visible { 0. } else { 1000. };
    transform.set_translation_xyz(x_pos, y_pos, z_pos);
    let sprite = world.read_resource::<RooftopSpriteResource>().rooftop_sprite_render.clone();
    world.create_entity()
        .with(sprite)
        .with(transform)
        .with(RooftopComponent::default().with_collision_enabled(enabled))
        .with_static_physical_entity(
            CollisionShape2::<f32, BodyPose2<f32>, ()>::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(
                ROOFTOP_COLLISION_RECTANGLE_WIDTH,
                ROOFTOP_COLLISION_RECTANGLE_HEIGHT).into()
            ),
            BodyPose2::<f32>::new(
                Point2::new(x_pos, y_pos),
                Basis2::one(),
            ),
            PhysicalEntity::default().with_gravity_scale(0.),
            Mass2::infinite()
        )
        .build()
}

/// Draws a path of rooftops.
///
/// This can be used to create an initial path of rooftops
/// in the visible area. For an endless jumper, a system
/// can create rooftops at somewhat random locations.
///
/// # Parameters
///
/// - `world`: The ECS `World` of our application.
pub fn draw_rooftop_path(world: &mut World) {
    let screen_dimensions = (*world.read_resource::<amethyst::window::ScreenDimensions>()).clone();
    let mut xpos = 0.5;
    let mut ypos = 0.2; // location of base roof

    ypos += 0.30;
    let _ = create_rooftop_entity(
        world,
        &screen_dimensions,
        Point2::new(xpos, ypos),
        false,
        false,
    );
    xpos += 0.05;
    ypos += 0.30;
    let _ = create_rooftop_entity(
        world,
        &screen_dimensions,
        Point2::new(xpos, ypos),
        false,
        false
    );
    xpos -= 0.12;
    ypos += 0.30;
    let _ = create_rooftop_entity(
        world,
        &screen_dimensions,
        Point2::new(xpos, ypos),
        false,
        false
    );
    xpos += 0.05;
    ypos += 0.30;
    let _ = create_rooftop_entity(
        world,
        &screen_dimensions,
        Point2::new(xpos, ypos),
        false,
        false
    );
    xpos += 0.1;
    ypos += 0.30;
    let _ = create_rooftop_entity(
        world,
        &screen_dimensions,
        Point2::new(xpos, ypos),
        false,
        false
    );
    xpos -= 0.05;
    ypos += 0.30;
    let _ = create_rooftop_entity(
        world,
        &screen_dimensions,
        Point2::new(xpos, ypos),
        false,
        false
    );
}