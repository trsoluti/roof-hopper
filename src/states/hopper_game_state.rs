//! The running state for the game

use amethyst::State;
use crate::game_data::GameData;
use crate::game_events::HopperGameStateEvent;

/// The state that manages the running of the game
#[derive(Default)]
pub struct HopperGameState;

impl<'a, 'b> State<GameData<'a, 'b>, HopperGameStateEvent> for HopperGameState {

}