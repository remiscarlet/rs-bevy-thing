use bevy::prelude::*;

use crate::state_manager::{DebugState, GameSceneState};

mod debug_action;
mod game_action;
mod view_action;

pub struct EventHandlerPlugin;

impl Plugin for EventHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_action::debug_action_handler)
            .add_systems(
                Update,
                (
                    game_action::game_action_handler,
                    view_action::view_action_handler,
                )
                    .run_if(in_state(GameSceneState::InGame)),
            );
    }
}
