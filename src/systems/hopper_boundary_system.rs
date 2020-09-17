//! This system keeps track of the hopper's location
//! and raises an event if she goes out of bounds

use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{System, SystemData, ReadStorage, Write,ReadExpect};
use amethyst::core::Transform;
use amethyst::core::shrev::EventChannel;
use crate::game_events::GameEvent;
use amethyst::window::ScreenDimensions;
use crate::resources::{PlayerEntityResource, MainCameraEntityResource};

const HOPPER_LOWER_Y_BOUNDARY: f32 = -10.;

/// A system that determines if the hopper has gone out of bounds
#[derive(SystemDesc, Default)]
pub struct HopperBoundarySystem;

impl <'a> System<'a> for HopperBoundarySystem {
    type SystemData = (
        ReadExpect<'a, PlayerEntityResource>,
        ReadExpect<'a, MainCameraEntityResource>,
        ReadExpect<'a, ScreenDimensions>,
        ReadStorage<'a, Transform>,
        Write<'a, EventChannel<GameEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            main_camera_entity,
            screen_dimensions,
            transforms,
            mut event_channel,
        ) = data;

        let hopper_y_position = transforms
            .get(player_entity.player_entity)
            .unwrap()
            .translation()
            .y;

        let camera_y_position = transforms
            .get(main_camera_entity.main_camera_entity)
            .unwrap()
            .translation()
            .y;

        let lower_boundary = camera_y_position
            - screen_dimensions.height() / 2.
            - HOPPER_LOWER_Y_BOUNDARY;

        //+log::info!("Player y pos: {:?}, boundary = {:?} - {:?} - {:?} = {:?}",
        //+    hopper_y_position,
        //+    camera_y_position,
        //+    screen_dimensions.height() / 2.,
        //+    HOPPER_LOWER_Y_BOUNDARY,
        //+    lower_boundary,
        //+);

        if hopper_y_position < lower_boundary {
            log::info!("Hopper below lower y boundary. Raising event.");
            event_channel.single_write(GameEvent::PlayerOutOfBounds)
        }
    }
}