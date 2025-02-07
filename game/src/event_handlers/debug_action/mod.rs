use bevy::prelude::*;

use crate::{
    input::DebugAction,
    state_manager::{self, DebugState},
};

pub fn debug_action_handler(
    curr_game_state: Res<State<DebugState>>,
    mut next_game_state: ResMut<NextState<DebugState>>,
    mut action_reader: EventReader<DebugAction>,
) {
    for event in action_reader.read() {
        if DebugAction::ToggleDebug == *event {
            let new_state = match curr_game_state.get() {
                DebugState::DebugDisabled => DebugState::DebugEnabled,
                DebugState::DebugEnabled => DebugState::DebugDisabled,
            };
            next_game_state.set(new_state);
        }
    }
}
