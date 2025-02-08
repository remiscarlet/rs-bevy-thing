use bevy::prelude::*;

use crate::action_handlers::{DebugAction, GameAction, ViewAction};

use super::systems;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DebugAction>()
            .add_event::<GameAction>()
            .add_event::<ViewAction>()
            .add_systems(Update, systems::process_debug_input)
            .add_systems(Update, systems::process_game_input)
            .add_systems(Update, systems::process_view_input);
    }
}
