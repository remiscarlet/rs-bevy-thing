use bevy::prelude::*;

mod plugin;

pub use plugin::ConfigPlugin;

#[derive(Resource)]
pub struct Config {
    pub hex_size: f32,
    pub camera_scale: f32,
    pub highlight_color: Color,
    pub default_highlight_color: Color,
}
