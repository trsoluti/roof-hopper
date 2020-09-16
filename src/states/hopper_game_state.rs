//! The running state for the game

use amethyst::{State, StateData, Trans};
use amethyst::prelude::{World, WorldExt};
use amethyst::core::Transform;
use amethyst::core::ecs::Join;
use crate::game_data::GameData;
use crate::game_events::HopperGameStateEvent;
use amethyst::input::{is_close_requested, is_key_down, get_key};
use amethyst::winit::VirtualKeyCode;
use amethyst_rhusics::time_sync;
use crate::resources::PlayerEntityResource;
use crate::components::HopperComponent;
use amethyst::renderer::rendy::wsi::winit::ElementState;

/// Identifies that the object with this transform
/// is "hidden" behind the camera and can be
/// exposed.
const HIDDEN_Z_POSITION: f32 = 1000.;
const VISIBLE_Z_POSITION: f32 = 0.;
const GRAVITY_ACCELERATION: f32 = -9.8;

/// The state that manages the running of the game
#[derive(Default)]
pub struct HopperGameState;

impl<'a, 'b> State<GameData<'a, 'b>, HopperGameStateEvent> for HopperGameState {
    /// Runs before first frame.
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // call a system-like method to make all the sprites visible.
        // The reason why it's a method and not a system
        // is that it's only ever run once.
        self.make_sprites_visible(world);

        // Turn on the gravity for our physics system
        self.turn_on_gravity(world);
    }

    /// Handles any events that have occurred this frame.
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: HopperGameStateEvent,
    ) -> Trans<GameData<'a, 'b>, HopperGameStateEvent> {
        if let HopperGameStateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
            // Look for our key events
            if let Some((keycode, state)) = get_key(&event) {
                if [VirtualKeyCode::A,
                    VirtualKeyCode::Left,
                    VirtualKeyCode::D,
                    VirtualKeyCode::Right,
                    VirtualKeyCode::X,
                    VirtualKeyCode::Space,
                ].contains(&keycode) && state == ElementState::Pressed {
                    let player_entity = data.world.read_resource::<PlayerEntityResource>().player_entity;
                    if let Some(hopper_component) = data.world.write_storage::<HopperComponent>().get_mut(player_entity) {
                        if keycode == VirtualKeyCode::X || keycode == VirtualKeyCode::Space && hopper_component.can_jump() {
                            hopper_component.start_jump()
                        }
                        else if keycode == VirtualKeyCode::A || keycode == VirtualKeyCode::Left && hopper_component.can_nudge() {
                            hopper_component.start_nudge(true)
                        }
                        else if keycode == VirtualKeyCode::D || keycode == VirtualKeyCode::Right && hopper_component.can_nudge() {
                            hopper_component.start_nudge(false)
                        }
                    }
                }
            }
        }
        Trans::None
    }

    /// Updates the physics time component and dispatches
    /// the systems.
    ///
    /// Note the call to "game" dispatcher. This means
    /// that systems under this dispatch group will only
    /// be run in the Hopper Game State.
    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, HopperGameStateEvent> {
        time_sync(data.world);
        data.data.update(&data.world, &[data.data.get_dispatcher_id("game")]);
        Trans::None
    }
}

// Subsidiary functions
impl HopperGameState {
    /// Makes any sprites with an 'invisible' setting
    /// for z-value 'visible, by changing the value to 0.
    ///
    /// It shows how you can do system-like things
    /// like going through all the transforms
    /// in a method called from the state.
    ///
    /// # Parameters
    ///
    /// - `world`: The ECS `World` of our application.
    fn make_sprites_visible(&self, world: &mut World) {
        let mut transforms = world.write_storage::<Transform>();
        for transform in (&mut transforms).join() {
            if transform.translation().z == HIDDEN_Z_POSITION {
                transform.set_translation_z(VISIBLE_Z_POSITION);
            }
        }
    }
    /// Turns on the gravity setting of the physics world parameters.
    ///
    /// # Parameters
    ///
    /// - `world`: The ECS `World` of our application.
    fn turn_on_gravity(&self, world: &mut World) {
        // Create a type alias to of what our 2D world parameters will be.
        // (This really should be part of amethyst-rhusics)
        use amethyst_rhusics::rhusics_core::WorldParameters;
        use cgmath::{Point2, EuclideanSpace, Vector2};
        type MyWorldParameters = WorldParameters<<Point2<f32> as EuclideanSpace>::Diff, <Point2<f32> as EuclideanSpace>::Scalar>;

        let mut world_parameters = world.write_resource::<MyWorldParameters>();
        *world_parameters = MyWorldParameters::new(Vector2::new(0.0, GRAVITY_ACCELERATION));
    }
}