use bevy::prelude::*;

use super::{systems, SelectedHexEntity};

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedHexEntity(None))
            .add_systems(Update, systems::initialize_map_hex)
            .add_systems(Update, systems::log_new_hex)
            .add_systems(Update, systems::select_clicked_hex)
            .add_systems(Update, systems::highlight_hovered_hex)
            .add_systems(Update, systems::move_selected_hex)
            .add_systems(Update, systems::update_sprite_colors);
    }
}
