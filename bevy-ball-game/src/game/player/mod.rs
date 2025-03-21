use bevy::prelude::*;
use systems::*;
// use systems::*;
pub mod components;
mod systems;

pub const PLAYER_SPEED: f32 = 500.0;
// TODO: either query the sprite size or hard set the size to be this.
// There is no contract holding this to be always true.
pub const PLAYER_SIZE: f32 = 64.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
    Collision,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            .add_startup_system(spawn_player)
            .add_system(player_movement.in_set(PlayerSystemSet::Movement))
            .add_system(confine_player_movement.in_set(PlayerSystemSet::Confinement))
            .add_system(enemy_hit_player.in_set(PlayerSystemSet::Collision))
            .add_system(player_hit_star.in_set(PlayerSystemSet::Collision));
    }
}
