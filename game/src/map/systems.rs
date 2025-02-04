use bevy::prelude::*;

use super::Map;

pub fn spawn_one_map(mut commands: Commands) {
    println!("Spawning one map!");
    commands.spawn((
        Map { width: 10, height: 5 },
        Name::new("My Great Map"),
    ));
}