use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

pub const PLAYER_SPEED: f32 = 500.0;
// TODO: either query the sprite size or hard set the size to be this.
// There is no contract holding this to be always true.
pub const PLAYER_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_system(update_camera_on_resize)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .run();
}

#[derive(Component)]
pub struct Player {}
#[derive(Component)]
pub struct Enemy {}

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

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            // I suppose this can be thought of as a tag
            Enemy {},
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
