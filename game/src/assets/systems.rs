use bevy::prelude::*;

use super::GameAssets;

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = GameAssets {
        hex_tile: asset_server.load("tiles/tile.png"),
        unit_dot: asset_server.load("tiles/unit_dot.png"),
    };
    commands.insert_resource(assets);
}
