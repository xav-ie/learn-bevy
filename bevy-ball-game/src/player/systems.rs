use super::{components::*, PLAYER_SIZE, PLAYER_SPEED};
use crate::{
    enemy::{components::Enemy, ENEMY_SIZE},
    events::*,
    score::resources::Score,
    star::{components::Star, STAR_SIZE},
};
use bevy::{prelude::*, window::PrimaryWindow};

/// Spawn single player Component
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

/// Enable player to move with WASD/arrow keys
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    // The Transform here comes from the SpriteBundle above. Because we spawned
    // the Player as a Bundle to the SpriteBundle, the Player is selectable
    // with the Transform Component
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let keys_pressed = keyboard_input.get_pressed();
        keys_pressed.into_iter().for_each(|key| match key {
            KeyCode::Up | KeyCode::W => {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }
            KeyCode::Left | KeyCode::A => {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }
            KeyCode::Down | KeyCode::S => {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }
            KeyCode::Right | KeyCode::D => {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }
            _ => {}
        });

        if direction.length() > 0.0 {
            direction = direction.normalize()
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

/// Confine the player to the window bounds
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok(mut transform) = player_query.get_single_mut() {
        let half_player_size = PLAYER_SIZE / 2.0;
        let min_x = 0.0 + half_player_size;
        let min_y = 0.0 + half_player_size;
        let max_x = window.width() - half_player_size;
        let max_y = window.height() - half_player_size;

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

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = enemy_transform
                .translation
                .distance(player_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Game over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = star_transform
                .translation
                .distance(player_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            if distance < player_radius + star_radius {
                score.value += 1;
                let sound_effect = asset_server.load("../assets/audio/pluck_001.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }
        }
    }
}
