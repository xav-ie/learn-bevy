use bevy::prelude::*;
pub const SPAWN_STAR_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct SpawnStarTimer {
    pub timer: Timer,
}

impl Default for SpawnStarTimer {
    fn default() -> SpawnStarTimer {
        SpawnStarTimer {
            timer: Timer::from_seconds(SPAWN_STAR_TIME, TimerMode::Repeating),
        }
    }
}
