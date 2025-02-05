use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion},
    math::VectorSpace,
    prelude::*,
};

use crate::GameState;

use super::{InputAction, InputEvent};

pub fn process_button_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut input_writer: EventWriter<InputEvent>,
    game_state: Res<State<GameState>>, // Assume we have some game state
) {
    let mut movement = Vec2::ZERO;

    if keys.just_pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    }
    if keys.just_pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }
    if keys.just_pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keys.just_pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }

    if movement != Vec2::ZERO {
        match game_state.get() {
            GameState::InGame => {
                println!("Sending MovePlayer: {:?}", movement);
                input_writer.send(InputEvent(InputAction::MovePlayer(movement)));
            },
            GameState::MainMenu => {
                println!("Sending NavigateMenu: {:?}", movement);
                input_writer.send(InputEvent(InputAction::NavigateMenu(movement)));
            }
        };
    }
}

pub fn process_cursor_moved(
    mut cursor_moved_reader: EventReader<CursorMoved>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut input_writer: EventWriter<InputEvent>,
    game_state: Res<State<GameState>>,
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
            (GameState::InGame, true) => {
                println!("Sending movement: {:?}", movement);
                input_writer.send(InputEvent(InputAction::DragCamera(movement)));
            }
            _ => (),
        };
    }
}

pub fn process_cursor_clicked(
    windows: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut input_writer: EventWriter<InputEvent>,
    game_state: Res<State<GameState>>,
) {
    if let Ok(window) = windows.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            match (game_state.get()) {
                (GameState::InGame) => {
                    for mouse_button in mouse_buttons.get_just_pressed() {
                        println!("Sending game click: {:?}", mouse_button);
                        input_writer.send(InputEvent(InputAction::GameClick(*mouse_button, cursor_pos)));
                    }
                },
                (GameState::MainMenu) => {
                    for mouse_button in mouse_buttons.get_just_pressed() {
                        println!("Sending menu click: {:?}", mouse_button);
                        input_writer.send(InputEvent(InputAction::MenuClick(*mouse_button, cursor_pos)));
                    }
                }
            };
        }
    }
}