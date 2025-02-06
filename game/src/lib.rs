use std::collections::HashSet;

use bevy::prelude::*;

use space_editor::SpaceEditorPlugin;

use crate::assets::GameAssetPlugin;
use crate::camera::CameraPlugin;
use crate::config::ConfigPlugin;
use crate::console::GameConsolePlugin;
use crate::hex::HexGridPlugin;
use crate::input::InputPlugin;
use crate::map::MapPlugin;

mod assets;
mod camera;
mod config;
mod console;
mod hex;
mod input;
mod map;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    MainMenu,
    InGame,
}

// Implement `FromWorld` so `init_state` works
impl FromWorld for GameState {
    fn from_world(_world: &mut World) -> Self {
        GameState::InGame
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(CameraPlugin)
            .add_plugins(InputPlugin)
            .add_plugins(GameAssetPlugin)
            .add_plugins(HexGridPlugin)
            .add_plugins(ConfigPlugin)
            .add_plugins(MapPlugin)
            .add_plugins(GameConsolePlugin)
            .add_systems(OnEnter(GameState::InGame), on_ingame_enter);

        #[cfg(feature = "devtools")]
        {
            println!("'devtools' feature was enabled - Adding SpaceEditorPlugin!");
            app.add_plugins(SpaceEditorPlugin);
        }
    }
}
fn on_ingame_enter(mut commands: Commands) {
    println!("Entered InGame state!");

    commands.spawn_empty(); // Dummy command to ensure the queue isn't empty
}
