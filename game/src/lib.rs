use std::collections::HashSet;

use bevy::prelude::*;

use action_handlers::EventHandlerPlugin;
use space_editor::SpaceEditorPlugin;

use crate::assets::GameAssetPlugin;
use crate::camera::CameraPlugin;
use crate::console::DebugConsolePlugin;
use crate::hex::HexGridPlugin;
use crate::input::InputPlugin;
use crate::map::MapPlugin;
use crate::state_manager::{GameSceneState, StatePlugin};
use crate::units::UnitsPlugin;

mod action_handlers;
mod assets;
mod camera;
mod console;
mod hex;
mod input;
mod map;
mod state_manager;
mod units;
mod utils;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DebugConsolePlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(InputPlugin)
            .add_plugins(GameAssetPlugin)
            .add_plugins(HexGridPlugin)
            .add_plugins(StatePlugin)
            .add_plugins(MapPlugin)
            .add_plugins(EventHandlerPlugin)
            .add_plugins(UnitsPlugin)
            .add_systems(OnEnter(GameSceneState::InGame), on_ingame_enter);

        // Can't add both bevy-console and space_editor at the same time.
        // Both utilize the Egui plugin, meaning if both are enabled Bevy panics due to adding duplicate plugins.
        // #[cfg(feature = "devmode")]
        // {
        //     println!("'devmode' feature was enabled - Adding SpaceEditorPlugin!");
        //     app.add_plugins(SpaceEditorPlugin);
        // }
    }
}
fn on_ingame_enter(mut commands: Commands) {
    println!("Entered InGame state!");

    commands.spawn_empty(); // Dummy command to ensure the queue isn't empty
}
