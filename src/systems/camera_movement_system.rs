//! A system to reposition the camera when the hopper goes above midpoint

use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{System, WriteStorage, ReadExpect, ReadStorage, Join};
use amethyst::core::ecs::SystemData;
use amethyst::core::Transform;
use crate::resources::{PlayerEntityResource, MainCameraEntityResource};
use crate::components::BackgroundComponent;

/// A system that moves the camera along
/// with the hopper, so it appears the rooftops
/// are falling
///
/// The camera is only moved when the hopper
/// tries to rise above its center point,
/// so there is some hopper movement on the screen.
#[derive(SystemDesc, Default)]
pub struct CameraMovementSystem;

impl <'a> System<'a> for CameraMovementSystem {
    type SystemData = (
        ReadExpect<'a, PlayerEntityResource>,
        ReadExpect<'a, MainCameraEntityResource>,
        ReadStorage<'a, BackgroundComponent>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            main_camera_entity,
            background_components,
            mut transforms
        ) = data;

        let hopper_y_position = transforms
            .get(player_entity.player_entity)
            .unwrap()
            .translation()
            .y;

        let camera_transform = transforms
            .get_mut(main_camera_entity.main_camera_entity)
            .unwrap();

        // Determine the hopper position
        let above = hopper_y_position - camera_transform.translation().y;

        // Move the camera if the hopper is above it
        if above > 0. {
            camera_transform.set_translation_y(camera_transform.translation().y + above);

            // Adjust each background similarly
            for (_background_component, transform) in (&background_components, &mut transforms).join() {
                transform.set_translation_y(transform.translation().y + above);
            }
        }
    }
}