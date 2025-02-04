use bevy::prelude::*;

use crate::hex::HexGridPlugin;
use crate::map::MapPlugin;

mod hex;
mod map;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HexGridPlugin)
            .add_plugins(MapPlugin);
    }
}