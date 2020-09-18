//! A system to ensure the hopper stays at rest instead of bouncing up

use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{System, ReadExpect, ReadStorage, WriteStorage, SystemData};
use crate::components::HopperComponent;
use crate::resources::PlayerEntityResource;
use amethyst_rhusics::rhusics_core::NextFrame;
use amethyst_rhusics::rhusics_core::physics2d::Velocity2;
use cgmath::{Vector2};
use crate::config::GAME_CONFIGURATION;

/// A system to ensure the hopper stays on the roof
/// until the user presses Jump,
/// instead of bouncing back up automatically.
///
/// This system is necessary because Amethyst-Rhusics
/// currently has no mechanism to stop the rebound
/// when two objects collide.
///
/// (Actually there is, by changing the parameters
/// of the Physics Material.)
#[derive(SystemDesc, Default)]
pub struct HopperRestSystem;

impl<'a> System<'a> for HopperRestSystem {
    type SystemData = (
        ReadExpect<'a, PlayerEntityResource>,
        ReadStorage<'a, HopperComponent>,
        WriteStorage<'a, NextFrame<Velocity2<f32>>>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            hopper_components,
            mut next_frame_velocity_components,
        ) = data;

        // This is an example of a system that doesn't have a join loop.
        // We only have one player, so we just update its state
        //
        // We do it in a system to take advantage of parallelism.

        // Get the hopper's collision state and hopper component
        let hopper_component = hopper_components.get(player_entity.player_entity).unwrap();
        let mut next_frame_velocity_component = next_frame_velocity_components.get_mut(player_entity.player_entity).unwrap();

        // Override the Next Frame Velocity set by the physics system
        // if we are in bouncing or resting state and there is no jump force in place.
        if hopper_component.can_jump() && hopper_component.jump_force < f32::EPSILON {
            next_frame_velocity_component.value = Velocity2::new(
                Vector2::new(0., -GAME_CONFIGURATION.downward_pressure),
                0.
            )
        }
    }
}