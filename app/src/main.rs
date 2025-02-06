use bevy::prelude::*;

use space_editor::SpaceEditorPlugin;

use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(GamePlugin)
        .run();
}
