mod components;
mod systems;

use bevy::prelude::*;
use systems::{
    interactions::{
        interact_with_main_menu_button, interact_with_quit_button, interact_with_resume_button,
    },
    layout::{despawn_pause_menu, spawn_pause_menu},
};

use crate::{game::SimulationState, AppState};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_pause_menu
                .in_schedule(OnEnter(SimulationState::Paused))
                .run_if(in_state(AppState::Game)),
        )
        .add_system(despawn_pause_menu.in_schedule(OnExit(SimulationState::Paused)))
        .add_system(despawn_pause_menu.in_schedule(OnExit(AppState::Game)))
        .add_systems(
            (
                interact_with_resume_button,
                interact_with_main_menu_button,
                interact_with_quit_button,
            )
                .in_set(OnUpdate(SimulationState::Paused)),
        );
    }
}
