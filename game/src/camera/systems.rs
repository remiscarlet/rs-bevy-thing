use bevy::prelude::*;
use space_editor::prelude::*;
use space_editor::space_prefab::component::camera::PlaymodeCamera;

use crate::{
    input::{InputAction, InputEvent},
    state_manager::ConfigState,
};

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

pub fn move_camera(
    time: Res<Time>,
    config: Res<ConfigState>,
    mut camera_query: Query<&mut Transform, With<GameCameraMarker>>,
    mut input_reader: EventReader<InputEvent>,
) {
    for event in input_reader.read() {
        if let InputEvent(InputAction::DragCamera(direction)) = event {
            let mut camera_transform = camera_query.single_mut();
            camera_transform.translation.x -=
                direction.x * time.delta_secs() * config.camera_drag_scale;
            camera_transform.translation.y +=
                direction.y * time.delta_secs() * config.camera_drag_scale;
        }
    }
}
