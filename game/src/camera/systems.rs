use bevy::prelude::*;
use space_editor::prelude::*;
use space_editor::space_prefab::component::camera::PlaymodeCamera;

use crate::{input::ViewAction, state_manager::ConfigState};

#[derive(Component)]
pub struct GameCameraMarker;

#[derive(Component)]
pub struct DebugCameraMarker;

pub fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        GameCameraMarker,
        PlaymodeCamera::default(),
    ));

    // #[cfg(feature = "devmode")]
    // {
    //     commands.set_state(EditorState::Editor);
    // }
}
