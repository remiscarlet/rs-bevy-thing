use bevy::prelude::*;

use crate::{camera::GameCameraMarker, input::ViewAction, state_manager::ConfigState};

pub fn view_action_event_handler(
    mut camera_query: Query<&mut Transform, With<GameCameraMarker>>,
    mut action_reader: EventReader<ViewAction>,
    time: Res<Time>,
    config: Res<ConfigState>,
) {
    for event in action_reader.read() {
        if let ViewAction::DragCamera(direction) = event {
            let mut camera_transform = camera_query.single_mut();
            camera_transform.translation.x -=
                direction.x * time.delta_secs() * config.camera_drag_scale;
            camera_transform.translation.y +=
                direction.y * time.delta_secs() * config.camera_drag_scale;
        }
    }
}
