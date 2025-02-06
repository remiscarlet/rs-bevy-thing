use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleConfiguration, ConsolePlugin};

use super::systems;
use systems::ExampleCommand;

pub struct GameConsolePlugin;

impl Plugin for GameConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsolePlugin)
            .insert_resource(ConsoleConfiguration {
                // override config here
                ..Default::default()
            })
            .add_console_command::<ExampleCommand, _>(systems::example_command);
    }
}
