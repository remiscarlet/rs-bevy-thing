use bevy::prelude::*;

use crate::{config::Config, map::Map};

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

                commands.spawn((
                    Sprite::from_image(game_assets.hex_tile.clone()),
                    Transform::from_xyz(position.x, position.y, 0.0),
                    hex,
                    Name::new(format!("Hex ({}, {})", q, r)),
                )).set_parent(map_entity); // Attach to the Map entity
            }
        }
    }
}