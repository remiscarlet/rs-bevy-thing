use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::{command, Parser};

use crate::{
    hex::{Hex, Selected},
    map::MapTile,
    state_manager::{ConfigState, GameRuntimeState},
    utils::process_console_command,
};

use super::GameAction;

#[derive(Parser, ConsoleCommand)]
#[command(name = "select_clicked_hex")]
pub struct SelectClickedHex {
    x: f32,
    y: f32,
}
pub fn select_clicked_hex_command(
    mut action_writer: EventWriter<GameAction>,
    mut clicked_cmd: ConsoleCommand<SelectClickedHex>,
) {
    if let Some(cmd) = process_console_command(clicked_cmd) {
        action_writer.send(GameAction::Click(Vec2::new(cmd.x, cmd.y)));
    }
}

pub fn select_clicked_hex(
    mut commands: &mut Commands,
    tile_query: &Query<(Entity, &Hex, &MapTile)>,
    config_state: &Res<ConfigState>,
    mut runtime_state: &mut ResMut<GameRuntimeState>,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor_pos: &Vec2,
) {
    let mut clicked_hex_entity: Option<Entity> = None;

    if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, *cursor_pos) {
        let clicked_hex = Hex::world_to_hex(world_pos, config_state.hex_size);
        println!(
            "{:?} at {:?} (world_pos: {:?}). Clicked hex: {:?}",
            MouseButton::Left,
            cursor_pos,
            world_pos,
            clicked_hex
        );

        for (entity, hex, _map_tile_marker) in tile_query.iter() {
            if *hex == clicked_hex {
                clicked_hex_entity = Some(entity);
            }
        }

        if clicked_hex_entity != runtime_state.selected_hex_entity {
            if let Some(prev_selected_entity) = runtime_state.selected_hex_entity {
                commands.entity(prev_selected_entity).remove::<Selected>();
            }

            if let Some(clicked_entity) = clicked_hex_entity {
                commands.entity(clicked_entity).insert(Selected);
            }

            runtime_state.selected_hex_entity = clicked_hex_entity;
        }
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "move_selected_hex_command")]
pub struct MoveSelectedHexCommand {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    up: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    down: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    left: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    right: bool,
}

pub fn move_selected_hex_command(
    mut action_writer: EventWriter<GameAction>,
    mut move_cmd: ConsoleCommand<MoveSelectedHexCommand>,
) {
    if let Some(cmd) = process_console_command(move_cmd) {
        if cmd.up {
            action_writer.send(GameAction::Up);
        } else if cmd.down {
            action_writer.send(GameAction::Down);
        } else if cmd.left {
            action_writer.send(GameAction::Left);
        } else if cmd.right {
            action_writer.send(GameAction::Right);
        }
    }
}

pub fn move_selected_hex(
    mut commands: &mut Commands,
    tile_query: &Query<(Entity, &Hex, &MapTile)>,
    mut runtime_state: &mut ResMut<GameRuntimeState>,
    direction: Vec2,
) {
    if let Some(selected_entity) = runtime_state.selected_hex_entity {
        if let Ok((prev_selected_entity, prev_selected_hex, _map_tile_marker)) =
            tile_query.get(selected_entity)
        {
            let (mut q, mut r) = prev_selected_hex.to_axial();

            q += direction.x.round() as i32;
            r += direction.y.round() as i32;

            let new_selected_hex = Hex::from_axial(q, r);

            for (new_selected_entity, hex, _map_tile_marker) in tile_query.iter() {
                if *hex == new_selected_hex {
                    commands.entity(prev_selected_entity).remove::<Selected>();
                    commands.entity(new_selected_entity).insert(Selected);

                    runtime_state.selected_hex_entity = Some(new_selected_entity);

                    break;
                }
            }
        }
    }
}
