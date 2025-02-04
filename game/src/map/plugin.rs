use bevy::prelude::*;

use super::systems;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_one_map);
    }
}
