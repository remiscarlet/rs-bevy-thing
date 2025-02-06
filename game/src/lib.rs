use std::collections::HashSet;

use bevy::prelude::*;

use space_editor::SpaceEditorPlugin;

use crate::assets::GameAssetPlugin;
use crate::camera::CameraPlugin;
use crate::console::GameConsolePlugin;
use crate::hex::HexGridPlugin;
use crate::input::InputPlugin;
use crate::map::MapPlugin;
use crate::state::StatePlugin;

mod assets;
mod camera;
mod console;
mod hex;
mod input;
mod map;
mod state;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
enum GameSceneState {
    MainMenu,
    InGame,
}

// Implement `FromWorld` so `init_state` works
impl FromWorld for GameSceneState {
    fn from_world(_world: &mut World) -> Self {
        GameSceneState::InGame
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameSceneState>()
            .add_plugins(CameraPlugin)
            .add_plugins(InputPlugin)
            .add_plugins(GameAssetPlugin)
            .add_plugins(HexGridPlugin)
            .add_plugins(StatePlugin)
            .add_plugins(MapPlugin)
            .add_plugins(GameConsolePlugin)
            .add_systems(OnEnter(GameSceneState::InGame), on_ingame_enter);

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
