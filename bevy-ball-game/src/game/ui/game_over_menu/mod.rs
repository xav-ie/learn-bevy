mod components;
mod styles;
mod systems;

use bevy::prelude::*;
use systems::{interactions::*, layout::*, updates::*};

use crate::AppState;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_game_over_menu.in_schedule(OnEnter(AppState::GameOver)))
            .add_system(despawn_game_over_menu.in_schedule(OnExit(AppState::GameOver)))
            // Must always run to capture game over event
            // .add_system(update_final_score_text.after(OnEnter(AppState::GameOver)))
            .add_systems(
                (
                    interact_with_restart_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                    update_final_score_text.after(spawn_game_over_menu),
                )
                    .in_set(OnUpdate(AppState::GameOver)),
            );
    }
}
