use crate::game::SimulationState;
// use crate::components::*;
use crate::{events::*, AppState};
// use crate::resources::*;
use bevy::app::AppExit;
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

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

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Game {
            commands.insert_resource(NextState(Some(AppState::Game)));
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
        }
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

pub fn handle_game_over(mut commands: Commands, mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is {}.", event.score);
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}
