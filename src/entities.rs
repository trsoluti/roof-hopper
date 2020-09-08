//! Container for all the entities used by the application

use amethyst::prelude::{World, WorldExt};
use amethyst::assets::ProgressCounter;
use amethyst::window::ScreenDimensions;
use amethyst::renderer::ImageFormat;
use amethyst::renderer::SpriteRender;
use amethyst::assets::Loader;
use amethyst::assets::AssetStorage;
use amethyst::renderer::Texture;
use amethyst::renderer::SpriteSheet;
use amethyst::renderer::SpriteSheetFormat;
use amethyst::core::ecs::Entity;

pub mod background_entity;
pub mod camera_entity;
pub mod hopper_entity;
pub mod rooftop_entity;

/// Starts the load sequence for all the entities used our game.
///
/// # Parameters
///
///
/// - `world`: The ECS `World` of our application.
/// - `progress`: A counter that can inform us when all the entities are loaded.
///
/// # Returns
///
/// A set of four `Entity`s that can be used to register resources.
pub fn load_entities(
    world: &mut World,
    progress: &mut ProgressCounter
) -> (Entity, Entity, Entity, Entity) {
    let screen_dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
    let background_entity = background_entity::load_background_entity(
        world,
        progress,
        &screen_dimensions,
    );
    let hopper_entity = hopper_entity::load_hopper_entity(
        world,
        progress,
        &screen_dimensions,
    );
    let camera_entity = camera_entity::initialise_camera(world, &screen_dimensions);
    (background_entity, camera_entity, hopper_entity, background_entity)
}

/// Starts off the loading of a sprite.
///
/// # Parameters
///
///
/// - `world`: The ECS `World` of our application.
/// - `name`: The name of our entity.
///         In this application, all sprites are stored in separate files,
///         i.e. the sprite sheet is the same as the sprite file.
/// - `progress`: A counter that can inform us when all the entities are loaded.
///
/// # Returns
///
/// A handle that can be used as a component or a resource.
pub fn load_sprite(
    world: &mut World,
    name: &'static str,
    progress: &mut ProgressCounter
) -> SpriteRender {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        let sprite_name = format!("sprites/{}.png", name);
        loader.load(
            sprite_name,
            ImageFormat::default(),
            progress,
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        let sheet_name = format!("sprites/{}.ron", name);
        loader.load(
            sheet_name,
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
    SpriteRender {
        sprite_sheet: sheet_handle.clone(),
        sprite_number: 0,
    }
}

