mod game_over_menu;
mod hud;
mod pause_menu;

use bevy::prelude::*;

use game_over_menu::GameOverMenuPlugin;
use hud::HudPlugin;
use pause_menu::PauseMenuPlugin;

pub struct GameUIMenuPlugin;

impl Plugin for GameUIMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HudPlugin)
            .add_plugin(GameOverMenuPlugin)
            .add_plugin(PauseMenuPlugin);
    }
}
