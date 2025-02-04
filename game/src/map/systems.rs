use bevy::prelude::*;

use super::Map;

pub fn spawn_one_map(mut commands: Commands) {
    println!("Spawning one map!");
    commands.spawn((
        Map::new(10, 5),
        Name::new("My Great Map"),
        Transform::default(),
        Visibility::default(),
    ));
}
