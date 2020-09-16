//! Stores the event state for collision
//! for the entity to which it is attached.
//!
//! Collisions in amethyst-rhusics are only relayed
//! through events. This system converts it into a
//! state variable, either in collision or not in collision

use std::fmt::Debug;
use amethyst_rhusics::rhusics_ecs::collide2d::ContactEvent2;
use amethyst::core::ecs::Component;
use amethyst::core::ecs::VecStorage;

/// A component that stores the current contact state for the
/// given entity.
#[derive(Debug, Clone, Default)]
pub struct CollisionComponent {
    /// The current contact state. None = no contact.
    pub contact_event: Option<ContactEvent2<f32>>,
}

impl CollisionComponent {
    /// A convenience fn to ask the component if it is in collision
    /// or not.
    #[inline]
    pub fn is_in_collision(&self) -> bool { self.contact_event.is_some() }
}

impl Component for CollisionComponent {
    type Storage = VecStorage<Self>;
}