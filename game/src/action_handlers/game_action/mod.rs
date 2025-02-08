use bevy::{math::VectorSpace, prelude::*};
use bevy_console::ConsoleCommand;
use clap::{command, Parser};
use hex::{move_selected_hex, select_clicked_hex};

use crate::{
    hex::HexTile,
    state_manager::{ConfigState, GameRuntimeState},
};

mod hex;
pub use hex::*;

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

pub fn game_action_event_handler(
    mut commands: Commands,
    hex_query: Query<(Entity, &HexTile)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut action_reader: EventReader<GameAction>,
    config_state: Res<ConfigState>,
    mut runtime_state: ResMut<GameRuntimeState>,
) {
    let (camera, camera_transform) = camera_query.single();
    for event in action_reader.read() {
        match *event {
            GameAction::Click(cursor_pos) => {
                select_clicked_hex(
                    &mut commands,
                    &hex_query,
                    &config_state,
                    &mut runtime_state,
                    camera,
                    camera_transform,
                    &cursor_pos,
                );
            }
            GameAction::Right | GameAction::Left | GameAction::Up | GameAction::Down => {
                let direction = if let GameAction::Right = *event {
                    Vec2::new(1.0, 0.0)
                } else if let GameAction::Left = *event {
                    Vec2::new(-1.0, 0.0)
                } else if let GameAction::Up = *event {
                    Vec2::new(0.0, 1.0)
                } else {
                    Vec2::new(0.0, -1.0)
                };

                move_selected_hex(&mut commands, &hex_query, &mut runtime_state, direction);
            }
            _ => {
                println!("Unhandled GameAction: {:?}", event);
            }
        }
    }
}
