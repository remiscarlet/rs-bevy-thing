use bevy::prelude::*;

mod plugin;
mod systems;

pub use plugin::InputPlugin;

pub enum InputAction {
    MovePlayer(Vec2),
    MoveCamera(Vec2),
    NavigateMenu(Vec2),
}

#[derive(Event)]
pub struct InputEvent(pub InputAction);
