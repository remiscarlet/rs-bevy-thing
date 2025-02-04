use bevy::prelude::*;

use super::systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::initialize_camera)
            .add_systems(Update, systems::move_camera);
    }
}