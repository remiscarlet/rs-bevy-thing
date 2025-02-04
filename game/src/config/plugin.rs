use bevy::prelude::*;
use super::Config;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config {
            hex_size: 50.0,
        });
    }
}