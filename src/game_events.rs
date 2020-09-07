//! The Game Event container
//!
//! Game events are all siphoned through this container
//! for the main game loop to read

use amethyst::input::{StringBindings, BindingTypes, InputEvent};
use cgmath::BaseNum;
use std::fmt::Debug;
use amethyst::prelude::World;
use amethyst::ui::UiEvent;
use amethyst_rhusics::rhusics_ecs::physics2d::ContactEvent2;
use amethyst::core::EventReader;
use amethyst::core::ecs::ReaderId;
use amethyst::core::shrev::EventChannel;
use amethyst::derive::EventReader;
use amethyst::ecs::{Read, SystemData};
use amethyst::winit::Event;
use amethyst::SimpleState;

/// The different types of events handled by our
/// Game State
///
/// # Type Parameters
///
/// - `T`: The type assigned to the Actions for the
///        InputBundle or InputHandler.
/// - `S`: Scalar type (`f32` or `f64`)
#[derive(Clone, Debug, EventReader)]
#[reader(HopperGameStateEventReader)]
pub enum HopperGameStateEvent<T = StringBindings, S = f32>
    where
        T: BindingTypes + Clone,
        S: BaseNum + Clone + Copy + Debug + Send + Sync + 'static
{
    /// Events sent by the winit window.
    Window(Event),
    /// Events sent by the ui system.
    Ui(UiEvent),
    /// Events sent by the input system.
    Input(InputEvent<T>),
    /// Collision events (not currently used)
    Collision(ContactEvent2<S>),
    /// Events specific to this game
    GameEvent(GameEvent),
}

/// The set of events specific to this game
#[derive(Clone, Debug, PartialEq)]
pub enum GameEvent {
    /// The player has gone out of bounds
    PlayerOutOfBounds,
}