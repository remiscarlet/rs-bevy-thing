use bevy::prelude::*;

use super::systems;
use super::DebugAction;
use super::GameAction;
use super::ViewAction;

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
