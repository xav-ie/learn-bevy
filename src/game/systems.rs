use bevy::prelude::*;

use super::SimulationState;

pub fn clear_event<T: 'static + Send + Sync>(mut events: ResMut<Events<T>>) {
    events.clear();
}

pub fn pause_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Paused);
    println!("Simulation Paused.");
}

pub fn resume_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Running);
    println!("Simulation Running.");
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.0 {
            SimulationState::Running => {
                pause_simulation(next_simulation_state);
            }
            SimulationState::Paused => {
                resume_simulation(next_simulation_state);
            }
        }
    }
}
