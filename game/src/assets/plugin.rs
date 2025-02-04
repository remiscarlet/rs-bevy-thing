use bevy::prelude::*;

pub struct GameAssetPlugin;

use super::systems;

impl Plugin for GameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::load_assets);
    }
}
