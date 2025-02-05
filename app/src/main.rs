use bevy::prelude::*;

use space_editor::SpaceEditorPlugin;

use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
