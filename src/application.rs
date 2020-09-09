//! The application signature

use amethyst::CoreApplication;
use crate::game_events::{HopperGameStateEvent, HopperGameStateEventReader};

/// The signature for our Roof Hopper application,
/// including a custom state event.
///
/// # Type Parameters
///
/// - `T`: The type of our initial state, which complies to the `State` trait.
pub type RoofHopperApplication<'a, T> = CoreApplication<'a, T, HopperGameStateEvent, HopperGameStateEventReader>;
