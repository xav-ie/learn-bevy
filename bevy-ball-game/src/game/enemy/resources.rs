use bevy::prelude::*;
pub const SPAWN_ENEMY_TIME: f32 = 5.0;

#[derive(Resource)]
pub struct SpawnEnemyTimer {
    pub timer: Timer,
}

impl Default for SpawnEnemyTimer {
    fn default() -> SpawnEnemyTimer {
        SpawnEnemyTimer {
            timer: Timer::from_seconds(SPAWN_ENEMY_TIME, TimerMode::Repeating),
        }
    }
}
