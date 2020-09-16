//! A system to manage the three soaring states: Rising, Peaking and Falling,
//! based on the entity's velocity.

use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{System, WriteStorage, Join, SystemData, ReadStorage};
use crate::components::{HopperComponent, HopperState};
use amethyst_rhusics::rhusics_core::physics2d::Velocity2;
use amethyst_rhusics::rhusics_core::NextFrame;

const PEAKING_THRESHOLD: f32 = 1.2;

/// A system that syncs the three soaring states, Rising, Peaking and Falling,
/// to the hopper's velocity.
///
/// Note NextFrame<Velocity> is used, as Amethyst-Rhusics has already
/// calculated it (and because AR doesn't actually update Velocity :( )
///
/// This system needs to be run *after* the Next Frame Setup System
/// ("next frame").
#[derive(SystemDesc, Default)]
pub struct HopperSoaringSystem;

impl<'a> System<'a> for HopperSoaringSystem {
    type SystemData = (
        ReadStorage<'a, NextFrame<Velocity2<f32>>>,
        WriteStorage<'a, HopperComponent>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            next_frame_velocities,
            mut hopper_components,
        ) = data;

        // Go through the velocity-component pairs and update the state
        // according to the velocity.
        for (next_frame_velocity, hopper_component) in (&next_frame_velocities, &mut hopper_components).join() {
            // velocity only affects the soaring states
            if hopper_component.is_soaring() {
                let upward_velocity = next_frame_velocity.value.linear().y;
                hopper_component.hopper_state = if upward_velocity > PEAKING_THRESHOLD {
                    HopperState::Rising
                } else if upward_velocity < - PEAKING_THRESHOLD {
                    HopperState::Falling
                } else {
                    HopperState::Peaking
                };
                // Debug line in case you're not convinced this is working:
                //+log::info!("Hopper soaring state is {:?}", hopper_component.hopper_state);
            }
        }
    }
}