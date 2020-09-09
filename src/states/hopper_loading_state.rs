//! The loading state for the game

use amethyst::{assets::{ProgressCounter, Completion}, input::{is_close_requested, is_key_down, VirtualKeyCode}, State, StateData, Trans};
use crate::{
    game_data::GameData,
    game_events::HopperGameStateEvent,
};
use crate::states::HopperGameState;
use crate::resources::insert_resources;
use crate::resources::insert_player_entity_resources;
use crate::components::register_components;
use crate::entities::load_entities;
use crate::entities::rooftop_entity::draw_rooftop_path;

/// The state which manages the loading of all the entities.
#[derive(Default)]
pub struct HopperLoadingState {
    /// A counter that can inform us when all the entities are loaded.
    progress: ProgressCounter,
}

impl<'a, 'b> State<GameData<'a, 'b>, HopperGameStateEvent> for HopperLoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        insert_resources(world, &mut self.progress);
        register_components(world);
        let (_background, camera, hopper, _rooftop) = load_entities(
            world,
            &mut self.progress,
        );
        insert_player_entity_resources(world, hopper, camera);
        draw_rooftop_path(world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: HopperGameStateEvent,
    ) -> Trans<GameData<'a, 'b>, HopperGameStateEvent> {
        if let HopperGameStateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }

    // Keep updating until all the entities have been loaded
    fn update(
        &mut self,
        state_data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, HopperGameStateEvent> {
        state_data.data.update(&state_data.world, &[]);
        match self.progress.complete() {
            Completion::Failed => {
                eprintln!("Failed loading assets");
                Trans::Quit
            }
            Completion::Complete => {
                log::info!("Assets loaded, transiting to game state.");
                Trans::Switch(Box::new(HopperGameState::default()))
            }
            Completion::Loading => Trans::None,
        }
    }
}