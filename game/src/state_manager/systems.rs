use bevy::prelude::*;

use crate::input::{InputAction, InputEvent};

use super::debug;
use super::DebugState;

pub fn toggle_debug_state_system(
    curr_game_state: Res<State<DebugState>>,
    mut next_game_state: ResMut<NextState<DebugState>>,
    mut input_reader: EventReader<InputEvent>,
) {
    for event in input_reader.read() {
        if let InputEvent(InputAction::ToggleDebug) = event {
            debug::toggle_debug_state(&curr_game_state, &mut next_game_state);
        }
    }
}
