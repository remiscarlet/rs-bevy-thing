use super::{systems, ConfigState, DebugState, GameRuntimeState, GameSceneState};
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameSceneState>()
            .init_state::<DebugState>()
            .insert_resource(ConfigState {
                hex_size: 50.0,
                camera_drag_scale: 100.0,

                default_hex_color: Color::WHITE,
                highlighted_hex_color: Color::linear_rgba(255.0, 0.0, 0.0, 0.5),
                selected_hex_color: Color::linear_rgba(100.0, 0.0, 100.0, 0.5),
            })
            .insert_resource(GameRuntimeState { debug: false })
            .add_systems(Update, systems::toggle_debug_state_system);
    }
}
