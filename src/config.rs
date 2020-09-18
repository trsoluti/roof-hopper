//! The set of items used by the Game Designer to tune the game after the coding is complete (e.g. player speed).

use amethyst::config::Config;
use serde_derive::{Serialize, Deserialize};

/// "Constants" that control the game mechanics.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameConfiguration {
    /// The amount of gravity in the system.
    ///
    /// The units of this are unclear, so the
    /// default value is figured out by feel.
    #[serde(default="default_gravity_acceleration")]
    pub gravity_acceleration: f32,
    /// The z-position of the camera
    #[serde(default="default_camera_z_position")]
    pub camera_z_position: f32,
    /// The z position of invisible sprites
    ///
    /// This needs to be > camera_z_position
    /// to actually hide the sprite
    /// and also a value that will not be
    /// used for anything other than hiding sprites
    #[serde(default="default_hidden_z_position")]
    pub hidden_z_position: f32,
    /// The default position of visible sprites
    ///
    /// This needs to be < camera_z_position
    /// to show the sprite.
    #[serde(default="default_visible_z_position")]
    pub visible_z_position: f32,
    /// The position of the background sprite
    /// relative to the screen width
    /// (i.e. from 0.0 left to 0.1 right).
    #[serde(default="default_background_rel_x_position")]
    pub background_rel_x_position: f32,
    /// The position of the background sprite
    /// relative to the screen height
    /// (i.e. from 0.0 bottom to 0.1 top).
    #[serde(default="default_background_rel_y_position")]
    pub background_rel_y_position: f32,
    /// The z position of the background sprite
    ///
    /// This needs to be < visible_z_position
    #[serde(default="default_background_z_position")]
    pub background_z_position: f32,
    /// The scale of the background image
    #[serde(default="default_background_scale")]
    pub background_scale: f32,
    /// The name of the file containing our player (hopper) sprite
    #[serde(default="default_hopper_sprite_name")]
    pub hopper_sprite_name: String,
    /// The width of the hopper collision rectangle
    #[serde(default="default_hopper_collision_rectangle_width")]
    pub hopper_collision_rectangle_width: f32,
    /// The height of the hopper collision rectangle
    #[serde(default="default_hopper_collision_rectangle_height")]
    pub hopper_collision_rectangle_height: f32,
    /// The name of the file containing our rooftop sprite
    #[serde(default="default_rooftop_sprite_name")]
    pub rooftop_sprite_name: String,
    /// The width of the rooftop collision rectangle
    #[serde(default="default_rooftop_collision_rectangle_width")]
    pub rooftop_collision_rectangle_width: f32,
    /// The height of the rooftop collision rectangle
    #[serde(default="default_rooftop_collision_rectangle_height")]
    pub rooftop_collision_rectangle_height: f32,
    /// The x offset of the rooftop collision rectangle
    #[serde(default="default_rooftop_collision_rectangle_offset_x")]
    pub rooftop_collision_rectangle_offset_x: f32,
    /// The y offset of the rooftop collision rectangle
    #[serde(default="default_rooftop_collision_rectangle_offset_y")]
    pub rooftop_collision_rectangle_offset_y: f32,
    /// How hard to jump when the jump button pressed
    #[serde(default="default_jump_force")]
    pub jump_force: f32,
    /// How much to force left/right when the appropriate button pressed
    #[serde(default="default_sideways_force")]
    pub sideways_force: f32,
    /// How far below the screen the hopper needs to be
    /// before it is out of bounds
    #[serde(default="default_hopper_lower_y_boundary")]
    pub hopper_lower_y_boundary: f32,
    /// Number of frames to debounce the hopper
    #[serde(default="default_debouncing_frame_count")]
    pub debouncing_frame_count: u32,
    /// Max portion of the jump force per frame
    #[serde(default="default_max_jump_force_per_frame")]
    pub max_jump_force_per_frame: f32,
    /// Max portion of the left/right nudge force per frame
    #[serde(default="default_max_nudge_force_per_frame")]
    pub max_nudge_force_per_frame: f32,
    /// Pressure used to keep hopper on roof
    #[serde(default="default_downward_pressure")]
    pub downward_pressure: f32,
    /// Speed at which hopper transitions between rising, peaking and falling.
    #[serde(default="default_peaking_threshold")]
    pub peaking_threshold: f32,
    /// how far above the roof the hopper needs to be
    /// before roof collision is enabled.
    #[serde(default="default_hopper_position_leeway")]
    pub hopper_position_leeway: f32,
}

// Default values
const GRAVITY_ACCELERATION: f32 = -98.;
const CAMERA_Z_POSITION: f32 = 1.;
const HIDDEN_Z_POSITION: f32 = 1000.;
const VISIBLE_Z_POSITION: f32 = 0.;
const BACKGROUND_REL_X_POSITION: f32 = 0.;
const BACKGROUND_REL_Y_POSITION: f32 = 0.5;
const BACKGROUND_Z_POSITION: f32 = -1.;
const BACKGROUND_SCALE: f32 = 3.;
// These values are based on an analysis of the sprite we use as our player.
const HOPPER_COLLISION_RECTANGLE_WIDTH: f32 = 68./2.;
const HOPPER_COLLISION_RECTANGLE_HEIGHT: f32 = 171./2.;
// These values were created from analysis of the
// sprite. The rectangle is 101 x 171 but the visible part is
// only 101 x 81 and is at the bottom.
//
// Theoretically, the offset should then be (171-81)/2
//
// But in practice this is too much. /4 works, not sure why.
const ROOFTOP_COLLISION_RECTANGLE_WIDTH: f32 = 101.;
const ROOFTOP_COLLISION_RECTANGLE_HEIGHT: f32 = 81.;
const ROOFTOP_COLLISION_RECTANGLE_OFFSET_X: f32 = 0.;
const ROOFTOP_COLLISION_RECTANGLE_OFFSET_Y: f32 = -(171.-81.)/4.;
/// How hard to jump when the jump button pressed
const JUMP_FORCE: f32 = 16_800.;
/// How much to force left/right when the appropriate button pressed
const SIDEWAYS_FORCE: f32 = 1100.;
// How far below the screen before out of bounds
const HOPPER_LOWER_Y_BOUNDARY: f32 = -10.;
// # of frames to bounce
const DEBOUNCING_FRAME_COUNT: u32 = 5;
const MAX_JUMP_FORCE_PER_FRAME: f32 = 4000.;
const MAX_NUDGE_FORCE_PER_FRAME: f32 = 400.;
// used to keep the hopper from bouncing up.
const DOWNWARD_PRESSURE:f32 = 4.;
// to differentiate between rising, peaking and falling
const PEAKING_THRESHOLD: f32 = 1.2;
// how far above the roof the hopper needs to be
// before roof collision is enabled.
const HOPPER_POSITION_LEEWAY: f32 = 64.; // calculated by eyeballing the hopper
// (see hopper entity for this value)


// fns to load those default values
// if not present in the .ron file:
fn default_gravity_acceleration() -> f32 { GRAVITY_ACCELERATION }
fn default_camera_z_position() -> f32 { CAMERA_Z_POSITION }
fn default_hidden_z_position() -> f32 { HIDDEN_Z_POSITION }
fn default_visible_z_position() -> f32 { VISIBLE_Z_POSITION }
fn default_background_rel_x_position() -> f32 { BACKGROUND_REL_X_POSITION }
fn default_background_rel_y_position() -> f32 { BACKGROUND_REL_Y_POSITION }
fn default_background_z_position() -> f32 { BACKGROUND_Z_POSITION }
fn default_background_scale() -> f32 { BACKGROUND_SCALE }
fn default_hopper_sprite_name() -> String { "Character Cat Girl".to_string() }
fn default_hopper_collision_rectangle_width() -> f32 { HOPPER_COLLISION_RECTANGLE_WIDTH }
fn default_hopper_collision_rectangle_height() -> f32 { HOPPER_COLLISION_RECTANGLE_HEIGHT }
fn default_rooftop_sprite_name() -> String { "Roof North".to_string() }
fn default_rooftop_collision_rectangle_width() -> f32 { ROOFTOP_COLLISION_RECTANGLE_WIDTH }
fn default_rooftop_collision_rectangle_height() -> f32 { ROOFTOP_COLLISION_RECTANGLE_HEIGHT }
fn default_rooftop_collision_rectangle_offset_x() -> f32 { ROOFTOP_COLLISION_RECTANGLE_OFFSET_X }
fn default_rooftop_collision_rectangle_offset_y() -> f32 { ROOFTOP_COLLISION_RECTANGLE_OFFSET_Y }
fn default_jump_force() -> f32 { JUMP_FORCE }
fn default_sideways_force() -> f32 { SIDEWAYS_FORCE }
fn default_hopper_lower_y_boundary() -> f32 { HOPPER_LOWER_Y_BOUNDARY }
fn default_debouncing_frame_count() -> u32 { DEBOUNCING_FRAME_COUNT }
fn default_max_jump_force_per_frame() -> f32 { MAX_JUMP_FORCE_PER_FRAME }
fn default_max_nudge_force_per_frame() -> f32 { MAX_NUDGE_FORCE_PER_FRAME }
fn default_downward_pressure() -> f32 { DOWNWARD_PRESSURE }
fn default_peaking_threshold() -> f32 { PEAKING_THRESHOLD }
fn default_hopper_position_leeway() -> f32 { HOPPER_POSITION_LEEWAY }

impl Default for GameConfiguration {
    fn default() -> Self {
        GameConfiguration {
            gravity_acceleration: default_gravity_acceleration(),
            camera_z_position: default_camera_z_position(),
            hidden_z_position: default_hidden_z_position(),
            visible_z_position: default_visible_z_position(),
            background_rel_x_position: default_background_rel_x_position(),
            background_rel_y_position: default_background_rel_y_position(),
            background_z_position: default_background_z_position(),
            background_scale: default_background_scale(),
            hopper_sprite_name: default_hopper_sprite_name(),
            hopper_collision_rectangle_width: default_hopper_collision_rectangle_width(),
            hopper_collision_rectangle_height: default_hopper_collision_rectangle_height(),
            rooftop_sprite_name: default_rooftop_sprite_name(),
            rooftop_collision_rectangle_width: default_rooftop_collision_rectangle_width(),
            rooftop_collision_rectangle_height: default_rooftop_collision_rectangle_height(),
            rooftop_collision_rectangle_offset_x: default_rooftop_collision_rectangle_offset_x(),
            rooftop_collision_rectangle_offset_y: default_rooftop_collision_rectangle_offset_y(),
            jump_force: default_jump_force(),
            sideways_force: default_sideways_force(),
            hopper_lower_y_boundary: default_hopper_lower_y_boundary(),
            debouncing_frame_count: default_debouncing_frame_count(),
            max_jump_force_per_frame: default_max_jump_force_per_frame(),
            max_nudge_force_per_frame: default_max_nudge_force_per_frame(),
            downward_pressure: default_downward_pressure(),
            peaking_threshold: default_peaking_threshold(),
            hopper_position_leeway: default_hopper_position_leeway(),
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