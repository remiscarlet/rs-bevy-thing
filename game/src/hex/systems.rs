use bevy::prelude::*;

use crate::map::Map;

use super::Hex;

pub fn initialize_hex_map(
    mut commands: Commands,
    query: Query<(Entity, &Map, &Name), Added<Map>>
) {
    for (map_entity, map, name) in query.iter() {
        println!("Spawning tiles for map '{}' ({})", name, map_entity);
        for q in 0..map.width {
            for r in 0..map.height {
                commands.spawn((
                    Hex::new(i32::from(q), i32::from(r)),
                    Name::new(format!("Hex ({}, {})", q, r)),
                )).set_parent(map_entity); // Attach to the Map entity
            }
        }
    }
}