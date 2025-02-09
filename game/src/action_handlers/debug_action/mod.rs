use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand};
use clap::{command, Parser};

use crate::{
    state_manager::{self, DebugState},
    utils::process_console_command,
};

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
            println!("Calling toggle_debug from action handler");
            toggle_debug(&curr_game_state, &mut next_game_state)
        }
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "toggle_debug_command")]
pub struct ToggleDebugCommand;
pub fn toggle_debug_command(
    toggle_cmd: ConsoleCommand<ToggleDebugCommand>,
    mut action_writer: EventWriter<DebugAction>,
) {
    if let Some(_cmd) = process_console_command(toggle_cmd) {
        println!("Calling toggle_debug from command handler");
        action_writer.send(DebugAction::ToggleDebug);
    }
}

fn toggle_debug(
    curr_game_state: &Res<State<DebugState>>,
    next_game_state: &mut ResMut<NextState<DebugState>>,
) {
    let new_state = match curr_game_state.get() {
        DebugState::DebugDisabled => DebugState::DebugEnabled,
        DebugState::DebugEnabled => DebugState::DebugDisabled,
    };
    println!("Toggling debug to: {:?}", &new_state);
    next_game_state.set(new_state);
}
