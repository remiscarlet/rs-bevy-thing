use bevy::prelude::*;

mod plugin;
mod systems;

pub use plugin::InputPlugin;

#[derive(Event, PartialEq, Debug)]
pub enum GameAction {
    Click(Vec2),
    Select,
    Cancel,
    Left,
    Right,
    Up,
    Down,
    Context,
    UnitDetails,
    TerrainDetails,
}

#[derive(Event, PartialEq, Debug)]
pub enum ViewAction {
    DragCamera(Vec2),
    ZoomIn(Vec2),
    ZoomOut(Vec2),
}

#[derive(Event, PartialEq, Debug)]
pub enum DebugAction {
    ToggleDebug,
    DebugOff,
    DebugOn,
}
