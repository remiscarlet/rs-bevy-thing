use bevy::prelude::*;

mod plugin;
mod systems;

#[derive(Component)]
pub struct Map {
    pub width: u16,
    pub height: u16,
}


pub use plugin::MapPlugin;


