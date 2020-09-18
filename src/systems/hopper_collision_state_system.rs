//! A system that manages the hopper state in relation to the roof.
//!
//! Amethyst-rhusics has no ability to control bounce, so we
//! have to force the object down until it's resting.

use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{System, ReadExpect, ReadStorage, WriteStorage, SystemData};
use crate::components::{CollisionComponent, HopperComponent, HopperState};
use crate::resources::PlayerEntityResource;
use crate::config::GAME_CONFIGURATION;


/// A system that "debounces" a collision between the hopper
/// and a rooftop
#[derive(SystemDesc, Default)]
pub struct HopperCollisionStateSystem;

impl<'a> System<'a> for HopperCollisionStateSystem {
    type SystemData = (
        ReadExpect<'a, PlayerEntityResource>,
        ReadStorage<'a, CollisionComponent>,
        WriteStorage<'a, HopperComponent>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            collision_components,
            mut hopper_components,
        ) = data;

        // This is an example of a system that doesn't have a join loop.
        // We only have one player, so we just update its state
        //
        // We do it in a system to take advantage of parallelism.

        // Get the hopper's collision state and hopper component
        let in_collision = collision_components
            .get(player_entity.player_entity)
            .unwrap()
            .is_in_collision();
        let mut hopper_component = hopper_components.get_mut(player_entity.player_entity).unwrap();
        let jump_force = hopper_component.jump_force;

        // Update the state based on the collision status and jump force
        hopper_component.hopper_state = if jump_force > f32::EPSILON {
            HopperState::Rising
        } else if in_collision {
            match hopper_component.hopper_state {
                HopperState::Rising | HopperState::Peaking | HopperState::Falling => HopperState::Bouncing(
                    GAME_CONFIGURATION.debouncing_frame_count
                ),
                HopperState::Bouncing(frames_left) => if frames_left > 1 {
                    HopperState::Bouncing(frames_left - 1)
                } else {
                    HopperState::Resting
                }
                HopperState::Resting => HopperState::Resting,
            }
        } else {
            match hopper_component.hopper_state {
                HopperState::Rising => HopperState::Rising,
                HopperState::Peaking => HopperState::Peaking,
                HopperState::Falling => HopperState::Falling,
                HopperState::Bouncing(frames_left) => if frames_left > GAME_CONFIGURATION.debouncing_frame_count * 2 {
                    HopperState::Rising
                } else {
                    HopperState::Bouncing(frames_left + 1)
                },
                HopperState::Resting => HopperState::Bouncing(1)
            }
        };
        //+log::info!("Hopper state set to {:?}, collision {:?}",
        //+ hopper_component.hopper_state, in_collision)
    }
}