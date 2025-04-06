use super::components::*;
use super::resources::*;
use super::NUMBER_OF_STARS;
use super::STAR_SIZE;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

/// Spawn `NUMBER_OF_STARS` Star Components
pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * (window.width() - STAR_SIZE) + STAR_SIZE / 2.0;
        let random_y = random::<f32>() * (window.height() - STAR_SIZE) + STAR_SIZE / 2.0;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            // I suppose this can be thought of as a tag
            Star {
                // direction: Vec2::new(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0)
                //     .normalize(),
            },
        ));
    }
}

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for star in star_query.iter() {
        commands.entity(star).despawn();
    }
}

pub fn tick_spawn_star_timer(mut spawn_star_timer: ResMut<SpawnStarTimer>, time: Res<Time>) {
    spawn_star_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    spawn_star_timer: Res<SpawnStarTimer>,
) {
    if spawn_star_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * (window.width() - STAR_SIZE) + STAR_SIZE / 2.0;
        let random_y = random::<f32>() * (window.height() - STAR_SIZE) + STAR_SIZE / 2.0;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}
