//! A simple jumping game using Entity-Component-Systems (ECS) and Amethyst.
//!
//! Each part of the game is documented, providing patterns you can use
//! to solve various problems in creating games that run under Amethyst.
//!
//! The implementation of this game uses the following features, additions,
//! and customizations from Amethyst:
//!
//! - Amethyst 2D template
//! - Amethyst-Rhusics ECS physics systems, including:
//!    - collisions
//!    - gravity
//!    - applying force
//! - Extended event-handling
//! - Optional running of systems
//! - Integrating events and components
//! - Using empty components to identify entity types
//! - Using sprite handles as resources for multiple spawning

#![deny(missing_docs)]

use amethyst::utils::application_root_dir;
use amethyst_rhusics::DefaultPhysicsBundle2;
use amethyst::core::TransformBundle;
use amethyst::renderer::{RenderingBundle, RenderToWindow, RenderFlat2D};
use amethyst::renderer::types::DefaultBackend;
use crate::game_data::DispatchGroupBuilder;
use crate::bundle::GameBundle;
use crate::application::RoofHopperApplication;
use crate::states::HopperLoadingState;

// Components of a standard Amethyst game, broken into modules:
mod application; // The definition of the Amethyst application
mod bundle;      // The bundle of systems used by the application
mod config;      // Game configuration information
mod components;  // All ECS Components
mod entities;    // All ECS Entities
mod game_data;   // The part that controls which systems run when
mod game_events; // Game events
mod resources;   // All game-wide components
mod states;      // All game states or "scenes"
mod systems;     // All ECS Systems

/// The entry point for our application.
///
/// This function loads all the configuration information
/// and gets all the systems in place.
///
/// Then it passes control to our version of the Amethyst Application
/// to go into our first state.
fn main() -> amethyst::Result<()> {
    // Start up the system logger, so we can use log::info!()
    amethyst::start_logger(Default::default());

    // Point our config loaders to the appropriate directory:
    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let _ = &config::GAME_CONFIGURATION; // will cause a load of the cfg info

    let game_data = game_data::GameDataBuilder::default()
        .with_bundle(DefaultPhysicsBundle2::<()>::new()/*.with_spatial()*/)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_dispatch_group(
            DispatchGroupBuilder::new("game")
                .with_bundle(GameBundle)?
        )?
        ;
    let mut game = RoofHopperApplication::new(
        resources,
        HopperLoadingState::default(),
        game_data,
    )?;

    game.run();

    Ok(())
}
