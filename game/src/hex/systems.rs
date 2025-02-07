use std::u32;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    camera::GameCameraMarker,
    hex::Selected,
    input::{InputAction, InputEvent},
    map::Map,
    state_manager::ConfigState,
};

use super::{Hex, Highlighted, SelectedHexEntity};
use crate::assets::GameAssets;

pub fn log_new_hex(query: Query<(&Hex, &Transform), Added<Hex>>) {
    for (Hex { q, r, s }, transform) in query.iter() {
        println!(
            "Spawning new Hex entity at ({}, {}, {}) in ({}, {})",
            q, r, s, transform.translation.x, transform.translation.y
        );
    }
}

pub fn initialize_map_hex(
    mut commands: Commands,
    config: Res<ConfigState>,
    query: Query<(Entity, &Map, &Name), Added<Map>>,
    game_assets: Res<GameAssets>,
) {
    for (map_entity, map, name) in query.iter() {
        println!("Spawning tiles for map '{}' ({})", name, map_entity);
        for q in 0..map.width {
            for r in 0..map.height {
                let hex = Hex::new(i32::from(q), i32::from(r));
                let position = Hex::hex_to_world_position(hex, config.hex_size);

                let mut sprite = Sprite::from_image(game_assets.hex_tile.clone());
                sprite.custom_size = Some(Vec2::new(
                    config.hex_size * 3f32.sqrt(),
                    config.hex_size * 2f32,
                ));

                commands
                    .spawn((
                        sprite,
                        Transform::from_xyz(position.x, position.y, 0.0),
                        hex,
                        Name::new(format!("Hex ({}, {})", q, r)),
                    ))
                    .set_parent(map_entity); // Attach to the Map entity
            }
        }
    }
}

pub fn highlight_hovered_hex(
    mut commands: Commands,
    mut query: Query<(Entity, &Hex)>,
    mut previous_highlighted: Local<Option<Entity>>,
    config: Res<ConfigState>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform), With<GameCameraMarker>>,
) {
    let mut closest_dist: u32 = u32::MAX;
    let mut closest_hex_entity: Option<Entity> = None;

    let (camera, camera_transform) = *camera_query;

    if let Ok(window) = windows.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                let cursor_hex = Hex::world_to_hex(world_pos, config.hex_size);

                // Find Entity for tile we're hovering over
                for (entity, hex) in query.iter_mut() {
                    let distance = cursor_hex.distance_to(hex);
                    if distance < closest_dist {
                        closest_dist = distance;
                        closest_hex_entity = Some(entity);
                    }
                }

                if *previous_highlighted != closest_hex_entity {
                    if let Some(prev_entity) = *previous_highlighted {
                        commands.entity(prev_entity).remove::<Highlighted>();
                    }

                    if let Some(hex_entity) = closest_hex_entity {
                        commands.entity(hex_entity).insert(Highlighted);
                    }

                    *previous_highlighted = closest_hex_entity;
                }
            }
        }
    }
}

pub fn select_clicked_hex(
    mut commands: Commands,
    hex_query: Query<(Entity, &Hex)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut input_reader: EventReader<InputEvent>,
    mut selected_hex_entity: ResMut<SelectedHexEntity>,
    config: Res<ConfigState>,
) {
    let mut clicked_hex_entity: Option<Entity> = None;

    let (camera, camera_transform) = camera_query.single();
    for event in input_reader.read() {
        if let InputEvent(InputAction::GameClick(MouseButton::Left, cursor_pos)) = event {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, *cursor_pos) {
                let clicked_hex = Hex::world_to_hex(world_pos, config.hex_size);
                println!(
                    "{:?} at {:?} (world_pos: {:?}). Clicked hex: {:?}",
                    MouseButton::Left,
                    cursor_pos,
                    world_pos,
                    clicked_hex
                );

                for (entity, hex) in hex_query.iter() {
                    if *hex == clicked_hex {
                        clicked_hex_entity = Some(entity);
                    }
                }

                if *selected_hex_entity != SelectedHexEntity(clicked_hex_entity) {
                    if let SelectedHexEntity(Some(prev_selected_entity)) = *selected_hex_entity {
                        commands.entity(prev_selected_entity).remove::<Selected>();
                    }

                    if let Some(clicked_entity) = clicked_hex_entity {
                        commands.entity(clicked_entity).insert(Selected);
                    }

                    *selected_hex_entity = SelectedHexEntity(clicked_hex_entity);
                }
            }
        }
    }
}

pub fn move_selected_hex(
    mut commands: Commands,
    hex_query: Query<(Entity, &Hex)>,
    mut input_reader: EventReader<InputEvent>,
    mut selected_hex_entity: ResMut<SelectedHexEntity>,
) {
    for event in input_reader.read() {
        if let InputEvent(InputAction::MovePlayer(direction)) = event {
            if let SelectedHexEntity(Some(selected_entity)) = *selected_hex_entity {
                if let Ok((prev_selected_entity, prev_selected_hex)) =
                    hex_query.get(selected_entity)
                {
                    let (mut q, mut r) = prev_selected_hex.to_axial();

                    q += direction.x.round() as i32;
                    r += direction.y.round() as i32;

                    let new_selected_hex = Hex::from_axial(q, r);

                    for (new_selected_entity, hex) in hex_query.iter() {
                        if *hex == new_selected_hex {
                            commands.entity(prev_selected_entity).remove::<Selected>();
                            commands.entity(new_selected_entity).insert(Selected);

                            *selected_hex_entity = SelectedHexEntity(Some(new_selected_entity));

                            break;
                        }
                    }
                }
            }
        }
    }
}

pub fn update_sprite_colors(
    mut query: Query<(&mut Sprite, Option<&Highlighted>, Option<&Selected>)>,
    config: Res<ConfigState>,
) {
    for (mut sprite, is_highlighted, is_selected) in query.iter_mut() {
        if is_selected.is_some() {
            sprite.color = config.selected_hex_color;
        } else if is_highlighted.is_some() {
            sprite.color = config.highlighted_hex_color;
        } else {
            sprite.color = config.default_hex_color;
        }
    }
}
