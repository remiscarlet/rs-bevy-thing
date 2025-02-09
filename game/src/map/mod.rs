use bevy::prelude::*;

mod plugin;
mod systems;

pub use plugin::MapPlugin;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Map {
    pub width: u16,
    pub height: u16,
}

impl Map {
    pub fn new(width: u16, height: u16) -> Self {
        println!(
            "Spawning new Map entity with dimensions ({} x {})",
            width, height
        );
        Self { width, height }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapTile;
impl MapTile {
    pub fn new() -> Self {
        println!("Spawning new MapTile");
        Self {}
    }
}
