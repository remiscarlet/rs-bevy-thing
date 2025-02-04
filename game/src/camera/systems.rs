use bevy::prelude::*;

use crate::{
    config::Config,
    input::{InputAction, InputEvent},
};

#[derive(Component)]
pub struct GameCamera;

pub fn initialize_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), GameCamera));
}

pub fn move_camera(
    time: Res<Time>,
    config: Res<Config>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
    mut input_reader: EventReader<InputEvent>,
) {
    for event in input_reader.read() {
        if let InputEvent(InputAction::MoveCamera(direction)) = event {
            let mut camera_transform = camera_query.single_mut();
            camera_transform.translation.x -= direction.x * time.delta_secs() * config.camera_scale;
            camera_transform.translation.y += direction.y * time.delta_secs() * config.camera_scale;
        }
    }
}
