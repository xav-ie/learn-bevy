pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;
mod ui;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use ui::GameUIMenuPlugin;

use crate::{events::GameOver, AppState};

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            // This alternative to add_event allows the game over screen to capture the event
            // https://bevy-cheatbook.github.io/patterns/manual-event-clear.html
            .init_resource::<Events<GameOver>>()
            .add_system(clear_event::<GameOver>.in_schedule(OnExit(AppState::GameOver)))
            .add_system(pause_simulation.in_schedule(OnExit(AppState::Game)))
            .add_system(resume_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_plugin(PlayerPlugin)
            .add_plugin(StarPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(GameUIMenuPlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
