//! Game States
//!
//! States are similar to "scenes" in other game engines.
//!
//! This game has two states:
//!
//! 1) Loading the sprites and
//! 2) Running the game
//!
//! It makes the transition from (1) to (2)
//! when the last sprite has been completely loaded.
//!
//! The physics and game systems are run only in state (2).

mod hopper_loading_state;
mod hopper_game_state;

pub use hopper_loading_state::HopperLoadingState;
pub use hopper_game_state::HopperGameState;