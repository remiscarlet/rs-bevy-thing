use bevy::prelude::*;

use super::systems;

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::initialize_map_hex)
            .add_systems(Update, systems::highlight_hovered_hex);
    }
}
