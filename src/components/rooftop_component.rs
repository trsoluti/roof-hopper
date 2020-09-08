//! A component to identify and to manage rooftop objects.

use amethyst::core::ecs::{Component, DenseVecStorage};

/// The component that identifies an entity as a rooftop.
///
/// Rooftops are pass-through platforms, meaning the player
/// can move upward through the roof (just once),
/// and then after will land on it.
#[derive(Debug, Copy, Clone, Default)]
pub struct RooftopComponent {
    /// True if the player can collide with (land on) the rooftop
    pub is_collision_enabled: bool,
}

impl RooftopComponent {
    /// Creates a rooftop component with collision set to the
    /// given `enabled` value.
    ///
    /// Returns the rooftop with the value set so it can be chained.
    #[inline]
    pub fn with_collision_enabled(mut self, enabled: bool) -> Self {
        self.is_collision_enabled = enabled;
        self
    }
}

impl Component for RooftopComponent {
    type Storage = DenseVecStorage<Self>;
}