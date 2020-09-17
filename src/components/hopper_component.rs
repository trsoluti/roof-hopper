//! Hopper component and associated data

use amethyst::core::ecs::{Component, DenseVecStorage};

/// How hard to jump when the jump button pressed
const JUMP_FORCE: f32 = 5_200.;
/// How much to force left/right when the appropriate button pressed
const SIDEWAYS_FORCE: f32 = 80.;

/// The possible states of our hopper
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HopperState {
    /// Hopper has jumped and is rising.
    Rising,
    /// Hopper has slowed jumping and is starting to fall.
    Peaking,
    /// Hopper is falling.
    Falling,
    /// Hopper has collided with the roof but is not yet resting on it.
    /// The value is the # of frames it has been in that state.
    Bouncing(u32),
    /// Hopper is resting on the roof
    Resting,
}
impl Default for HopperState {
    fn default() -> Self {
        HopperState::Falling
    }
}
impl HopperState {
    /// Returns true if the hopper is not bouncing/resting on the roof.
    #[inline]
    pub fn is_soaring(&self) -> bool {
        match self {
            Self::Rising => true,
            Self::Peaking => true,
            Self::Falling => true,
            _ => false
        }
    }
    /// Returns true of the hopper is in a state to respond to the jump event.
    #[inline]
    pub fn can_jump(&self) -> bool {
        match self {
            Self::Bouncing(_) => true,
            Self::Resting => true,
            _ => false,
        }
    }
    /// Returns true if the hopper is in a state to respond to a left/right
    /// nudge event.
    #[inline]
    pub fn can_nudge(&self) -> bool {
        match self {
            Self::Rising => true,
            _ => false,
        }
    }
}

/// The ninja state
#[derive(Default, Debug, Copy, Clone)]
pub struct HopperComponent {
    /// The current state of the hopper.
    pub hopper_state: HopperState,
    /// The current amount of nudge (left/right) force.
    pub nudge_force: f32,
    /// The current amount of jump force.
    pub jump_force: f32,
}

impl HopperComponent {
    /// Returns true if the hopper is not bouncing off / resting on a roof.
    #[inline]
    pub fn is_soaring(&self) -> bool { self.hopper_state.is_soaring() }
    #[inline]
    /// Returns true of the hopper is in a state to respond to the jump event.
    pub fn can_jump(&self) -> bool { self.hopper_state.can_jump() }
    /// Returns true if the hopper is in a state to respond to a left/right
    /// nudge event.
    #[inline]
    pub fn can_nudge(&self) -> bool { self.hopper_state.can_nudge() }
    /// Starts the jump sequence
    #[inline]
    pub fn start_jump(&mut self) { self.jump_force = JUMP_FORCE }
    /// Starts the nudge sequence
    pub fn start_nudge(&mut self, move_left: bool) {
        self.nudge_force = SIDEWAYS_FORCE * if move_left { -1. } else { 1. }
    }
}

impl Component for HopperComponent {
    type Storage = DenseVecStorage<Self>;
}