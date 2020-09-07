//! The set of items used by the Game Designer to tune the game after the coding is complete (e.g. player speed).

use amethyst::config::Config;
use serde_derive::{Serialize, Deserialize};

/// "Constants" that control the game mechanics.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameConfiguration {
        /// The amount of gravity in the system.
        #[serde(default)]
        pub gravity: f32,
}

// Default values
const GRAVITY: f32 = 0.98;

impl Default for GameConfiguration {
    fn default() -> Self {
        GameConfiguration {
            gravity: GRAVITY,
        }
    }
}

lazy_static::lazy_static! {
    /// The actual values for the [game configuration](struct.GameConfiguration.html).
    ///
    /// The game configuration is automatically loaded on startup
    /// from the file "game_config.ron" in resources.
    ///
    /// It's a good pattern for managing the game configuration
    /// so that the game designer can change parameters and balance
    /// the game without having to recompile the code.
    ///
    /// Using this method instead of asset loading allows the
    /// values to become available immediately and before World
    /// is properly established.
    ///
    /// This variable looks to the remaining code as if it were a set of constants.
    pub static ref GAME_CONFIGURATION: GameConfiguration = {
        let game_config_path = format!(
            "{}/resources/game_config.ron",
            env!("CARGO_MANIFEST_DIR")
        );
        GameConfiguration::load(&game_config_path).unwrap()
    };
}