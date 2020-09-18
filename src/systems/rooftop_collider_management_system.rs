//! A system to manage the state of the rooftop collider based on the position
//! of the player entity.

use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{System, ReadStorage, WriteStorage, ReadExpect};
use crate::components::RooftopComponent;
use amethyst::core::ecs::{Join, SystemData};
use amethyst::core::Transform;
use amethyst_rhusics::rhusics_core::collide2d::{CollisionShape2, BodyPose2};
use crate::resources::PlayerEntityResource;
use crate::config::GAME_CONFIGURATION;

/// A system that manages whether the rooftop is
/// available for collision or not.
///
/// The rooftop is available for collision
/// if the hopper has at any point risen above it
#[derive(SystemDesc, Default)]
pub struct RooftopColliderManagementSystem;

impl <'a> System<'a> for RooftopColliderManagementSystem {
    type SystemData = (
        ReadExpect<'a, PlayerEntityResource>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, RooftopComponent>,
        WriteStorage<'a, CollisionShape2::<f32, BodyPose2<f32>, ()>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            transforms,
            mut rooftop_components,
            mut collision_shapes
        ) = data;

        // Determine where the hopper is.
        let hopper_position = transforms
            .get(player_entity.player_entity)
            .unwrap()
            .translation()
            .y;
        //+log::info!("hopper y position is {:?}", hopper_position);
        // Update the rooftop component
        for (rooftop_component, rooftop_transform, collision_shape) in (&mut rooftop_components, &transforms, &mut collision_shapes).join() {
            //+log::info!("  rooftop is {:?} ({:?})", rooftop_transform.translation().y, rooftop_transform.translation().y + HOPPER_POSITION_LEEWAY);
            if hopper_position > rooftop_transform.translation().y + GAME_CONFIGURATION.hopper_position_leeway {
                rooftop_component.is_collision_enabled = true;
            }
            //+log::info!("  rooftop collision enabled = {:?}", rooftop_component.is_collision_enabled);
            // The is-collision-enabled could be initialized to true or false
            // by the part that creates the entity,
            // so this will pass the value on to the collision system
            // (which doesn't have the ability to start off disabled)
            collision_shape.enabled = rooftop_component.is_collision_enabled;
        }
    }
}