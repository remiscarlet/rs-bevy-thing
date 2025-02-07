use super::{ConfigState, DebugState, GameRuntimeState, GameSceneState};
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameSceneState>()
            .init_state::<DebugState>()
            .insert_resource(ConfigState {
                hex_size: 25.0,
                camera_drag_scale: 100.0,

                default_hex_color: Color::WHITE,
                highlighted_hex_color: Color::linear_rgba(1.0, 0.0, 0.0, 0.5),
                selected_hex_color: Color::linear_rgba(1.0, 0.0, 1.0, 0.5),
            })
            .insert_resource(GameRuntimeState {
                selected_hex_entity: None,
            });
    }
}
