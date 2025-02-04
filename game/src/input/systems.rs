use bevy::{input::mouse::MouseMotion, math::VectorSpace, prelude::*};

use crate::GameState;

use super::{InputAction, InputEvent};

pub fn process_button_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut input_writer: EventWriter<InputEvent>,
    game_state: Res<State<GameState>>, // Assume we have some game state
) {
    let mut movement = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) { movement.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { movement.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { movement.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { movement.x += 1.0; }

    if movement != Vec2::ZERO {
        match game_state.get() {
            GameState::InGame => input_writer.send(InputEvent(InputAction::MovePlayer(movement))),
            GameState::MainMenu => input_writer.send(InputEvent(InputAction::NavigateMenu(movement))),
        };
    }
}

pub fn process_mouse_motion(
    mut evr_motion: EventReader<MouseMotion>,
    mut input_writer: EventWriter<InputEvent>,
    game_state: Res<State<GameState>>, // Assume we have some game state
) {
    for ev in evr_motion.read() {
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);

        let movement = Vec2::from([
            ev.delta.x,
            ev.delta.y,
        ]);

        match game_state.get() {
            GameState::InGame => {
                input_writer.send(InputEvent(InputAction::MoveCamera(movement)));
            },
            _ => (),
        };
    }
}