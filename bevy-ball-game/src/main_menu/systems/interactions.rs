use bevy::{app::AppExit, prelude::*};

use crate::styles::*;
use crate::{
    button_interaction,
    main_menu::components::{PlayButton, QuitButton},
    AppState,
};

pub fn interact_with_play_button(
    mut button_query: button_interaction!(PlayButton),
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = ACTIVE_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: button_interaction!(QuitButton),
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = ACTIVE_BUTTON_COLOR.into();
                app_exit_event_writer.send_default();
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
