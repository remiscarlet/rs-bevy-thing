use super::Config;
use bevy::prelude::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config {
            hex_size: 50.0,
            camera_scale: 100.0,

            default_hex_color: Color::WHITE,
            highlighted_hex_color: Color::linear_rgba(255.0, 0.0, 0.0, 0.5),
            selected_hex_color: Color::linear_rgba(100.0, 0.0, 100.0, 0.5),
        });
    }
}
