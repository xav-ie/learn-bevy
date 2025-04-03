use bevy::prelude::*;
use resources::SpawnEnemyTimer;
use systems::*;

use crate::AppState;
pub mod components;
pub mod resources;
mod systems;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnEnemyTimer>()
            .add_system(enemy_movement)
            .add_system(update_enemy_direction.after(enemy_movement))
            .add_system(confine_enemy_movement.after(enemy_movement))
            .add_system(tick_spawn_enemy_timer)
            .add_system(spawn_enemies_over_time);
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)))
    }
}
