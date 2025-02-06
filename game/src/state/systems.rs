use bevy::prelude::*;

use crate::input::{InputAction, InputEvent};

use super::GameRuntimeState;

pub fn toggle_debug_state(
    mut game_state: ResMut<GameRuntimeState>,
    mut input_reader: EventReader<InputEvent>,
) {
    for event in input_reader.read() {
        if let InputEvent(InputAction::ToggleDebug) = event {
            game_state.debug = !(game_state.debug);
            println!(
                "Got ToggleDebug event! Debug is now: {:?}",
                game_state.debug
            );
        }
    }
}
