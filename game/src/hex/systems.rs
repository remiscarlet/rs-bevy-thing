use std::u32;

use bevy::{
    ecs::query::{QueryData, QuerySingleError},
    prelude::*,
    window::PrimaryWindow,
};

use crate::{camera::GameCamera, config::Config, map::Map};

use super::Hex;
use crate::assets::GameAssets;

pub fn initialize_map_hex(
    mut commands: Commands,
    config: Res<Config>,
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
                sprite.color = config.default_highlight_color;

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
    mut query: Query<(Entity, &Hex, &mut Sprite)>,
    mut previous_highlighted: Local<Option<Entity>>,
    config: Res<Config>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    let mut closest_dist: u32 = u32::MAX;
    let mut closest_hex_entity: Option<Entity> = None;

    let (camera, camera_transform) = *camera_query;

    if let Ok(window) = windows.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                let cursor_hex = Hex::world_to_hex(world_pos, config.hex_size);

                // Find Entity for tile we're hovering over
                for (entity, hex, _sprite) in query.iter_mut() {
                    let distance = cursor_hex.distance_to(hex);
                    if distance < closest_dist {
                        closest_dist = distance;
                        closest_hex_entity = Some(entity);
                    }
                }

                if *previous_highlighted != closest_hex_entity {
                    if let Some(prev_entity) = *previous_highlighted {
                        if let Ok((_entity, _hex, mut sprite)) = query.get_mut(prev_entity) {
                            sprite.color = config.default_highlight_color;
                        }
                    }

                    if let Some(hex_entity) = closest_hex_entity {
                        if let Ok((_entity, _hex, mut sprite)) = query.get_mut(hex_entity) {
                            sprite.color = config.highlight_color;
                        }
                    }

                    *previous_highlighted = closest_hex_entity;
                }
            }
        }
    }
}
