use bevy::prelude::*;
pub const SPAWN_STAR_TIME: f32 = 1.0;
pub const SPAWN_ENEMY_TIME: f32 = 5.0;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

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

#[derive(Resource)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores { scores: Vec::new() }
    }
}
