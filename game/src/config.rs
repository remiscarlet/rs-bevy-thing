use bevy::prelude::*;

mod plugin;

pub use plugin::ConfigPlugin;

#[derive(Resource)]
pub struct Config {
    pub hex_size: f32,
}