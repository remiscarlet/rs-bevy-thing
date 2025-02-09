//! This module handles all things "state" related.
//! This includes exposing states as both State and Resource types.
//! Most things should be in state Resources unless there's a specific purpose
//! in having it be a State, such as conditional system execution.

use bevy::prelude::*;

mod plugin;

pub use plugin::StatePlugin;

/*
 * Resources
 */

#[derive(Resource)]
pub struct ConfigState {
    /// Defined by the radius of the "outer circle" that would encompass our hexagon.
    /// See: https://www.redblobgames.com/grids/hexagons/#spacing
    pub hex_size: f32,
    pub camera_drag_scale: f32,

    pub highlighted_hex_color: Color,
    pub selected_hex_color: Color,
    pub default_hex_color: Color,
}

#[derive(Resource)]
pub struct GameRuntimeState {
    pub selected_hex_entity: Option<Entity>,
}

/*
 * States
 */

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum DebugState {
    DebugEnabled,
    DebugDisabled,
}

impl FromWorld for DebugState {
    fn from_world(_world: &mut World) -> Self {
        DebugState::DebugDisabled
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameSceneState {
    MainMenu,
    InGame,
}

// Implement `FromWorld` so `init_state` works
impl FromWorld for GameSceneState {
    fn from_world(_world: &mut World) -> Self {
        GameSceneState::InGame
    }
}
