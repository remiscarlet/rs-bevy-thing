use super::Config;
use bevy::prelude::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config {
            hex_size: 50.0,
            camera_scale: 100.0,
            highlight_color: Color::linear_rgb(255.0, 0.0, 0.0),
            default_highlight_color: Color::WHITE,
        });
    }
}
