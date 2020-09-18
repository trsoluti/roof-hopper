//! A system that handles the hopper hopping/nudging/not-hopping

use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{System, ReadExpect, WriteStorage, SystemData};
use crate::components::HopperComponent;
use crate::resources::PlayerEntityResource;
use amethyst_rhusics::rhusics_core::physics2d::ForceAccumulator2;
use cgmath::Vector2;
use crate::config::GAME_CONFIGURATION;

/// A system that converts the hopper jump/nudge force
/// into forces applied to the force accumulator.
#[derive(SystemDesc, Default)]
pub struct HopperJumpSystem;

impl<'a> System<'a> for HopperJumpSystem {
    type SystemData = (
        ReadExpect<'a, PlayerEntityResource>,
        WriteStorage<'a, HopperComponent>,
        WriteStorage<'a, ForceAccumulator2<f32>>,
    );
    fn run(&mut self, data: Self::SystemData) {
        // pull out the constants because we use them in a number of places
        let max_jump_force_per_frame = GAME_CONFIGURATION.max_jump_force_per_frame;
        let max_nudge_force_per_frame = GAME_CONFIGURATION.max_nudge_force_per_frame;
        let (
            player_entity,
            mut hopper_components,
            mut force_accumulator_components,
        ) = data;

        // This is an example of a system that doesn't have a join loop.
        // We only have one player, so we just update its state
        //
        // We do it in a system to take advantage of parallelism.

        // Get the hopper's collision state and hopper component
        let hopper_component = hopper_components.get_mut(player_entity.player_entity).unwrap();
        let force_accumulator_component = force_accumulator_components.get_mut(player_entity.player_entity).unwrap();

        // If we have some valid jump or nudge force:
        if hopper_component.jump_force > 0. || hopper_component.nudge_force != 0. {
            // Divide up the jump force over a number of frames
            // which can help get rid of bounce conditions, etc.
            let jump_force = if hopper_component.jump_force >= max_jump_force_per_frame { max_jump_force_per_frame } else { hopper_component.jump_force };
            let nudge_force = if hopper_component.nudge_force >= max_nudge_force_per_frame {
                max_nudge_force_per_frame
            } else if hopper_component.nudge_force <= -max_jump_force_per_frame {
                -max_nudge_force_per_frame
            } else {
                hopper_component.nudge_force
            };
            //+log::info!("Adding force({:?}, {:?})", jump_force, nudge_force);
            force_accumulator_component.add_force(
                Vector2::new(
                    nudge_force,
                    jump_force
                )
            );
            hopper_component.jump_force -= jump_force;
            hopper_component.nudge_force -= nudge_force;
        }
    }
}