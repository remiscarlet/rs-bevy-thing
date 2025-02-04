use bevy::prelude::*;

use super::InputEvent;
use super::systems;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>()
            .add_systems(Update, systems::process_button_input)
            .add_systems(Update, systems::process_mouse_motion);
    }
}