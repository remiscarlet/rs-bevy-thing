use bevy::prelude::*;

mod plugin;
mod systems;

pub use plugin::InputPlugin;

pub enum InputAction {
    MovePlayer(Vec2),
    GameClick(MouseButton, Vec2),
    MenuClick(MouseButton, Vec2),
    ToggleDebug,
    DragCamera(Vec2),
    NavigateMenu(Vec2),
}

#[derive(Event)]
pub struct InputEvent(pub InputAction);
