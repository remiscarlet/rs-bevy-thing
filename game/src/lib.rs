use bevy::prelude::*;

use crate::assets::GameAssetPlugin;
use crate::camera::CameraPlugin;
use crate::config::ConfigPlugin;
use crate::input::InputPlugin;
use crate::hex::HexGridPlugin;
use crate::map::MapPlugin;

mod assets;
mod camera;
mod config;
mod input;
mod hex;
mod map;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    MainMenu,
    InGame,
}

// Implement `FromWorld` so `init_state` works
impl FromWorld for GameState {
    fn from_world(_world: &mut World) -> Self {
        GameState::InGame // 👈 Choose a reasonable default
    }
}


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>() // ✅ Use `add_state`, NOT `init_state`
            .add_plugins(InputPlugin)
            .add_plugins(GameAssetPlugin)
            .add_plugins(HexGridPlugin)
            .add_plugins(ConfigPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(MapPlugin)
            .add_systems(OnEnter(GameState::InGame), on_ingame_enter);
    }
}
fn on_ingame_enter(mut commands: Commands) {
    println!("Entered InGame state!");

    commands.spawn_empty(); // Dummy command to ensure the queue isn't empty
}
