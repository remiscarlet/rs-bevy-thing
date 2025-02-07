use bevy::prelude::*;

use super::DebugState;

pub fn toggle_debug_state(
    curr_debug_state: &Res<State<DebugState>>,
    next_debug_state: &mut ResMut<NextState<DebugState>>,
) {
    let new_state = match *curr_debug_state.get() {
        DebugState::DebugDisabled => DebugState::DebugEnabled,
        DebugState::DebugEnabled => DebugState::DebugDisabled,
    };

    next_debug_state.set(new_state);
}
