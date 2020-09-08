//! A component to identify the background

use amethyst::core::ecs::{Component, NullStorage};

/// The component that identifies an entity as the background.
///
/// We move the background up with the camera instead of
/// lowering the rooftops and player.
///
/// The component is an empty type, so it uses `NullStorage`.
#[derive(Debug, Copy, Clone, Default)]
pub struct BackgroundComponent;

impl Component for BackgroundComponent {
    type Storage = NullStorage<Self>;
}