use bevy::prelude::*;

mod plugin;
mod systems;

pub use plugin::StatePlugin;

#[derive(Resource)]
pub struct ConfigState {
    /// Defined by the radius of the "outer circle" that would encompass our hexagon.
    /// See: https://www.redblobgames.com/grids/hexagons/#spacing
    pub hex_size: f32,
    pub camera_drag_scale: f32,

    pub highlighted_hex_color: Color,
    pub selected_hex_color: Color,
    pub default_hex_color: Color,
}

#[derive(Resource)]
pub struct GameRuntimeState {
    pub debug: bool,
}
