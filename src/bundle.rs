//! The bundle of systems that are part of the "game" dispatch group
//!
//! The following amethyst-rhusics systems are added:
//!
//! - `CurrentFrameUpdateSystem`: updates positions and velocities for the current frame, based on `NextFrame` values.
//! - `NextFrameSetupSystem`: sets up next-frame positions and velocities.
//! - `PoseTransformSyncSystem`: copies transform information from `BodyPose` in rhusics into `Transform`
//!                              in amethyst
//!
//! Note we use the 2D version of each system.
//!
//! Systems specifically for this game are added in [add_systems](../systems/fn.add_systems.html).

use amethyst::core::SystemBundle;
use amethyst::prelude::World;
use amethyst::core::ecs::DispatcherBuilder;
use amethyst_rhusics::rhusics_ecs::physics2d::{CurrentFrameUpdateSystem2, NextFrameSetupSystem2};
use amethyst_rhusics::rhusics_core::collide2d::BodyPose2;
use amethyst_rhusics::PoseTransformSyncSystem2;
use crate::systems::add_systems;

/// The game bundle
#[derive(Default)]
pub struct GameBundle;

impl <'a, 'b>SystemBundle<'a, 'b> for GameBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> amethyst::Result<()> {
        builder.add(CurrentFrameUpdateSystem2::<f32, BodyPose2<f32>>::new(), "current frame", &[]);
        builder.add(NextFrameSetupSystem2::<f32, BodyPose2<f32>>::new(), "next frame", &["current frame"]);
        builder.add(PoseTransformSyncSystem2::new().without_rotation(), "sync system", &[]);
        // Add our own systems:
        add_systems(world, builder);
        Ok(())
    }
}
