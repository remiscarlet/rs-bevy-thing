use bevy::prelude::*;

mod plugin;
mod systems;

#[derive(Component)]
pub struct Map {
    pub width: u16,
    pub height: u16,
}

impl Map {
    pub fn new(width: u16, height: u16) -> Self {
        println!("Spawning new Map entity with dimensions ({} x {})", width, height);
        Self { width, height }
    }
}

pub use plugin::MapPlugin;


