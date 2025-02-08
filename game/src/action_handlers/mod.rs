use bevy::prelude::*;
use bevy_console::AddConsoleCommand;
use debug_action::ToggleDebugCommand;
use game_action::{MoveSelectedHexCommand, SelectClickedHex};

use crate::state_manager::{DebugState, GameSceneState};

mod debug_action;
mod game_action;
mod view_action;

pub struct EventHandlerPlugin;

impl Plugin for EventHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_action::debug_action_event_handler)
            .add_systems(
                Update,
                (
                    game_action::game_action_event_handler,
                    view_action::view_action_event_handler,
                )
                    .run_if(in_state(GameSceneState::InGame)),
            )
            .add_console_command::<ToggleDebugCommand, _>(debug_action::toggle_debug_command)
            .add_console_command::<MoveSelectedHexCommand, _>(
                game_action::move_selected_hex_command,
            )
            .add_console_command::<SelectClickedHex, _>(game_action::select_clicked_hex_command);
    }
}
