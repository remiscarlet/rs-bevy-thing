use bevy::prelude::*;

mod plugin;

pub use plugin::ConfigPlugin;

#[derive(Resource)]
pub struct Config {
    pub hex_size: f32,
    pub camera_scale: f32,

    pub highlighted_hex_color: Color,
    pub selected_hex_color: Color,
    pub default_hex_color: Color,
}
