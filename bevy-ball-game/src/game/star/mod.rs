use bevy::prelude::*;
use resources::*;
use systems::*;
pub mod components;
pub mod resources;
mod systems;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnStarTimer>()
            .add_startup_system(spawn_stars)
            .add_system(tick_spawn_star_timer)
            .add_system(spawn_stars_over_time);
    }
}
