use bevy::prelude::*;

mod plugin;
mod systems;

pub use plugin::GameAssetPlugin;

#[derive(Resource)]
pub struct GameAssets {
    pub hex_tile: Handle<Image>,
    pub unit_dot: Handle<Image>,
}
