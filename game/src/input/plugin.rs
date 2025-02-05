use bevy::prelude::*;

use super::systems;
use super::InputEvent;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>()
            .add_systems(Update, systems::process_button_input)
            .add_systems(Update, systems::process_cursor_clicked)
            .add_systems(Update, systems::process_cursor_moved);
    }
}
