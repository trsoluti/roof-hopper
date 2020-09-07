//! A sprite descriptor that can be used to instantiate new entities of the rooftop

use amethyst::renderer::SpriteRender;

/// This resource is used to spawn new rooftop objects
/// during the game.
pub struct RooftopSpriteResource {
    /// The sprite renderer, which can be attached as a component
    /// to a new rooftop entity
    pub rooftop_sprite_render: SpriteRender,
}