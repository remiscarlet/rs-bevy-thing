use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand};
use clap::{command, Parser};

use crate::state_manager::{self, DebugState};

#[derive(Event, PartialEq, Debug)]
pub enum DebugAction {
    ToggleDebug,
    DebugOff,
    DebugOn,
}

pub fn debug_action_event_handler(
    curr_game_state: Res<State<DebugState>>,
    mut next_game_state: ResMut<NextState<DebugState>>,
    mut action_reader: EventReader<DebugAction>,
) {
    for event in action_reader.read() {
        if DebugAction::ToggleDebug == *event {
            toggle_debug(&curr_game_state, &mut next_game_state)
        }
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "toggle_debug_command")]
pub struct ToggleDebugCommand {
    x: f32,
    y: f32,
}
pub fn toggle_debug_command(
    curr_game_state: Res<State<DebugState>>,
    mut next_game_state: ResMut<NextState<DebugState>>,
) {
    toggle_debug(&curr_game_state, &mut next_game_state);
}

fn toggle_debug(
    curr_game_state: &Res<State<DebugState>>,
    next_game_state: &mut ResMut<NextState<DebugState>>,
) {
    let new_state = match curr_game_state.get() {
        DebugState::DebugDisabled => DebugState::DebugEnabled,
        DebugState::DebugEnabled => DebugState::DebugDisabled,
    };
    next_game_state.set(new_state);
}
