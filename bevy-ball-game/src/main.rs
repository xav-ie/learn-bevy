use bevy::{
    app::AppExit,
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use rand::thread_rng;
use rand::{random, seq::SliceRandom};

pub const PLAYER_SPEED: f32 = 500.0;
// TODO: either query the sprite size or hard set the size to be this.
// There is no contract holding this to be always true.
pub const PLAYER_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const SPAWN_STAR_TIME: f32 = 1.0;
pub const SPAWN_ENEMY_TIME: f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<SpawnStarTimer>()
        .init_resource::<SpawnEnemyTimer>()
        .add_event::<GameOver>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_stars)
        .add_system(update_camera_on_resize)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(player_hit_star)
        .add_system(update_score)
        .add_system(tick_spawn_star_timer)
        .add_system(tick_spawn_enemy_timer)
        .add_system(spawn_stars_over_time)
        .add_system(spawn_enemies_over_time)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

#[derive(Component)]
pub struct Player {}
#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}
#[derive(Component)]
pub struct Star {}

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

pub struct GameOver {
    pub score: u32,
}

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

// TODO: better way? maybe just update bevy?
pub fn update_camera_on_resize(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut resize_reader: EventReader<WindowResized>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for _event in resize_reader.iter() {
        let window = window_query.get_single().unwrap();
        if let Ok(mut transform) = camera_query.get_single_mut() {
            // Update the camera's position to center it in the new window
            transform.translation.x = window.width() / 2.0;
            transform.translation.y = window.height() / 2.0;
        }
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
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

/// Push enemies in the direction they are facing.
pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
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
            let sound_effects = vec![
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

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn tick_spawn_star_timer(mut spawn_star_timer: ResMut<SpawnStarTimer>, time: Res<Time>) {
    spawn_star_timer.timer.tick(time.delta());
}
pub fn tick_spawn_enemy_timer(mut spawn_enemy_timer: ResMut<SpawnEnemyTimer>, time: Res<Time>) {
    spawn_enemy_timer.timer.tick(time.delta());
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

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is {}.", event.score);
        app_exit_event_writer.send(AppExit);
    }
}
