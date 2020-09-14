//! A system to convert the existence/non-existence of a collision event
//! into a constantly-valid state component.

use amethyst::core::shrev::ReaderId;
use amethyst_rhusics::rhusics_ecs::collide2d::ContactEvent2;
use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{World, System, Read, WriteStorage, Join, SystemData};
use amethyst::core::ecs::shrev::EventChannel;
use crate::components::CollisionComponent;

/// A system that converts the contact event into a collision state
#[derive(SystemDesc, Default)]
pub struct EntityCollisionSystem {
    contact_reader: Option<ReaderId<ContactEvent2<f32>>>
}

impl EntityCollisionSystem {
    pub fn setup(mut self, world: &mut World) -> Self {
        self.contact_reader = Some(
            world.fetch_mut::<EventChannel<ContactEvent2<f32>>>()
                .register_reader()
        );
        self
    }
}

impl<'a> System<'a> for EntityCollisionSystem {
    type SystemData = (
        WriteStorage<'a, CollisionComponent>,
        Read<'a, EventChannel<ContactEvent2<f32>>>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            mut collision_components,
            contacts_channel
        ) = data;

        // Clear any existing collision components
        for mut collision_component in (&mut collision_components).join() {
            collision_component.contact_event = None
        }

        // Go through the collision events, and set the
        // appropriate collision component if the entity has one
        contacts_channel.read(&mut self.contact_reader.as_mut().unwrap())
            .for_each(|contact| {
                // go through each contact entity in the contact event
                // to see if it has an attached collision component
                [contact.bodies.0, contact.bodies.1].iter().for_each(|&body| {
                    if let Some(collision_component) = collision_components.get_mut(body) {
                        log::info!("Entity {:?} is in collision.", body);
                        collision_component.contact_event = Some(contact.clone())
                    }
                })
            })
    }
}