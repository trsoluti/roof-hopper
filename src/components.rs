//! Container for all the components used by the application

use amethyst::prelude::{World, WorldExt};

mod hopper_component;
mod rooftop_component;
mod background_component;

pub use hopper_component::{ HopperComponent, HopperState };
pub use rooftop_component::RooftopComponent;
pub use background_component::BackgroundComponent;


/// Registers all our custom component types with world to avoid
/// access errors.
///
/// # Parameters
///
/// `world`: The ECS `World` of our application.
pub fn register_components(world: &mut World) {
    world.register::<HopperComponent>();
    world.register::<RooftopComponent>();
    world.register::<BackgroundComponent>();
}