use crate::game::player::PLAYER_SIZE;

use super::components::*;
use super::resources::*;
use super::ENEMY_SIZE;
use super::ENEMY_SPEED;
use super::NUMBER_OF_ENEMIES;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::{random, seq::SliceRandom, thread_rng};

/// Spawn `NUMBER_OF_ENEMIES` Enemy Components
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * (window.width() - PLAYER_SIZE) + PLAYER_SIZE / 2.0;
        let random_y = random::<f32>() * (window.height() - PLAYER_SIZE) + PLAYER_SIZE / 2.0;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            // I suppose this can be thought of as a tag
            Enemy {
                direction: Vec2::new(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0)
                    .normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

/// Enable enemies to hit walls and reverse direction
pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let min_x = 0.0 + half_enemy_size;
    let min_y = 0.0 + half_enemy_size;
    let max_x = window.width() - half_enemy_size;
    let max_y = window.height() - half_enemy_size;
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;
        if translation.x < min_x || translation.x > max_x {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < min_y || translation.y > max_y {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sound_effects = [
                "audio/impactMetal_000.ogg",
                "audio/impactMetal_001.ogg",
                "audio/impactMetal_002.ogg",
                "audio/impactMetal_003.ogg",
                "audio/impactMetal_004.ogg",
            ];
            let mut rng = thread_rng();
            let sound_effect = sound_effects.choose(&mut rng).unwrap();
            let effect = asset_server.load(*sound_effect);

            audio.play(effect);
        }
    }
}

/// Confine the enemies to the window bounds
pub fn confine_enemy_movement(
    mut enemies_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for mut transform in enemies_query.iter_mut() {
        let half_enemy_size = ENEMY_SIZE / 2.0;
        let min_x = 0.0 + half_enemy_size;
        let min_y = 0.0 + half_enemy_size;
        let max_x = window.width() - half_enemy_size;
        let max_y = window.height() - half_enemy_size;

        let mut translation = transform.translation;
        if translation.x < min_x {
            translation.x = min_x;
        } else if translation.x > max_x {
            translation.x = max_x;
        }
        if translation.y < min_y {
            translation.y = min_y;
        } else if translation.y > max_y {
            translation.y = max_y;
        }
        transform.translation = translation;
    }
}

pub fn tick_spawn_enemy_timer(mut spawn_enemy_timer: ResMut<SpawnEnemyTimer>, time: Res<Time>) {
    spawn_enemy_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    spawn_enemy_timer: Res<SpawnEnemyTimer>,
) {
    if spawn_enemy_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * (window.width() - ENEMY_SIZE) + ENEMY_SIZE / 2.0;
        let random_y = random::<f32>() * (window.height() - ENEMY_SIZE) + ENEMY_SIZE / 2.0;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0)
                    .normalize(),
            },
        ));
    }
}

/// Push enemies in the direction they are facing.
pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}
