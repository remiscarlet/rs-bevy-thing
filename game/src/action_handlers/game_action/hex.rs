use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::{command, Parser};

use crate::{
    hex::{Hex, HexTile, Selected},
    state_manager::{ConfigState, GameRuntimeState},
    utils::process_console_command,
};

#[derive(Parser, ConsoleCommand)]
#[command(name = "select_clicked_hex")]
pub struct SelectClickedHex {
    x: f32,
    y: f32,
}
pub fn select_clicked_hex_command(
    mut commands: Commands,
    hex_query: Query<(Entity, &HexTile)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    config_state: Res<ConfigState>,
    mut runtime_state: ResMut<GameRuntimeState>,
    mut clicked_cmd: ConsoleCommand<SelectClickedHex>,
) {
    let (camera, camera_transform) = camera_query.single();
    match process_console_command(clicked_cmd) {
        Some(cmd) => {
            select_clicked_hex(
                &mut commands,
                &hex_query,
                &config_state,
                &mut runtime_state,
                &camera,
                &camera_transform,
                &Vec2::from([cmd.x, cmd.y]),
            );
        }
        _ => {}
    };
}

pub fn select_clicked_hex(
    mut commands: &mut Commands,
    hex_query: &Query<(Entity, &HexTile)>,
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

        for (entity, HexTile(hex)) in hex_query.iter() {
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
    mut commands: Commands,
    hex_query: Query<(Entity, &HexTile)>,
    mut runtime_state: ResMut<GameRuntimeState>,
    mut move_cmd: ConsoleCommand<MoveSelectedHexCommand>,
) {
    match process_console_command(move_cmd) {
        Some(cmd) => {
            println!(
                "Moving hex: up={} down={} left={} right={}",
                cmd.up, cmd.down, cmd.left, cmd.right
            );
            let mut direction = Vec2::ZERO;
            if cmd.up {
                direction.y += 1.0;
            }
            if cmd.down {
                direction.y -= 1.0;
            }
            if cmd.left {
                direction.x -= 1.0;
            }
            if cmd.right {
                direction.x += 1.0;
            }
            move_selected_hex(&mut commands, &hex_query, &mut runtime_state, direction);
        }
        _ => {}
    }
}

pub fn move_selected_hex(
    mut commands: &mut Commands,
    hex_query: &Query<(Entity, &HexTile)>,
    mut runtime_state: &mut ResMut<GameRuntimeState>,
    direction: Vec2,
) {
    if let Some(selected_entity) = runtime_state.selected_hex_entity {
        if let Ok((prev_selected_entity, HexTile(prev_selected_hex))) =
            hex_query.get(selected_entity)
        {
            let (mut q, mut r) = prev_selected_hex.to_axial();

            q += direction.x.round() as i32;
            r += direction.y.round() as i32;

            let new_selected_hex = Hex::from_axial(q, r);

            for (new_selected_entity, HexTile(hex)) in hex_query.iter() {
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
