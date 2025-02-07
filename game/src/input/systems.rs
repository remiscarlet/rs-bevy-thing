use bevy::prelude::*;

use crate::GameSceneState;

use super::{DebugAction, GameAction, ViewAction};

pub fn process_debug_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut action_writer: EventWriter<DebugAction>,
) {
    if keys.just_pressed(KeyCode::Backquote) {
        action_writer.send(DebugAction::ToggleDebug);
    }
}

pub fn process_game_input(
    windows: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut action_writer: EventWriter<GameAction>,
) {
    if keys.just_pressed(KeyCode::KeyW) {
        action_writer.send(GameAction::Up);
    }
    if keys.just_pressed(KeyCode::KeyS) {
        action_writer.send(GameAction::Down);
    }
    if keys.just_pressed(KeyCode::KeyA) {
        action_writer.send(GameAction::Left);
    }
    if keys.just_pressed(KeyCode::KeyD) {
        action_writer.send(GameAction::Right);
    }

    if let Ok(window) = windows.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            for mouse_button in mouse_buttons.get_just_pressed() {
                match mouse_button {
                    MouseButton::Left => {
                        println!("Sending game click: {:?}", mouse_button);
                        action_writer.send(GameAction::Click(cursor_pos));
                    }
                    _ => {
                        println!("Uncaught mouse button: {:?}", mouse_button);
                    }
                }
            }
        }
    }
}

pub fn process_view_input(
    mut cursor_moved_reader: EventReader<CursorMoved>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut action_writer: EventWriter<ViewAction>,
    game_state: Res<State<GameSceneState>>,
) {
    let mut movement = Vec2::ZERO;
    // Accumulate mouse movement from all events
    for ev in cursor_moved_reader.read() {
        if let Some(delta) = ev.delta {
            movement += delta;
        }
    }

    if movement != Vec2::ZERO {
        match (game_state.get(), mouse_buttons.pressed(MouseButton::Right)) {
            (GameSceneState::InGame, true) => {
                println!("Sending movement: {:?}", movement);
                action_writer.send(ViewAction::DragCamera(movement));
            }
            _ => (),
        };
    }
}
