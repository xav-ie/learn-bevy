pub mod enemy;
pub mod events;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use enemy::EnemyPlugin;
use events::*;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_startup_system(spawn_camera)
        .add_system(update_camera_on_resize)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_plugin(PlayerPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ScorePlugin)
        .run();
}
