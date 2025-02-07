use bevy::prelude::*;

use crate::{assets::GameAssets, map::Map, state_manager::DebugState};

use super::{systems, SelectedHexEntity};

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedHexEntity(None))
            .add_systems(Update, systems::initialize_map_hex)
            .add_systems(Update, systems::log_new_hex)
            .add_systems(Update, systems::highlight_hovered_hex)
            .add_systems(Update, systems::update_sprite_colors);

        app.add_systems(
            Update,
            (draw_sprite_bounding_boxes, debug_parent_transform)
                .run_if(in_state(DebugState::DebugEnabled)),
        );
        // .add_systems(Update, debug_sprite_size);
    }
}

fn debug_parent_transform(query: Query<(&GlobalTransform, &Name), With<Map>>) {
    for (transform, name) in query.iter() {
        let scale = transform.to_scale_rotation_translation().0;
        println!("{} Scale: {:?}", name, scale);
    }
}

fn debug_texture_size(assets: Res<Assets<Image>>, game_assets: Res<GameAssets>) {
    if let Some(texture) = assets.get(&game_assets.hex_tile) {
        println!("Texture size: {:?}", texture.size());
    }
}

fn debug_sprite_size(query: Query<&Sprite>) {
    for sprite in query.iter() {
        println!("Sprite size: {:?}", sprite.custom_size);
    }
}

fn debug_transform(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("Transform scale: {:?}", transform.scale);
    }
}

pub fn draw_sprite_bounding_boxes(
    mut gizmos: Gizmos,
    query: Query<(&GlobalTransform, &Sprite, Option<&Visibility>)>,
) {
    for (global_transform, sprite, visibility) in query.iter() {
        // Skip hidden objects
        if let Some(Visibility::Hidden) = visibility {
            continue;
        }

        let transform = global_transform.compute_transform();
        let size = sprite.custom_size.unwrap_or(Vec2::ONE);
        let half_size = size / 2.0;
        let center = transform.translation.truncate();

        let corners = [
            center + Vec2::new(-half_size.x, -half_size.y), // Bottom Left
            center + Vec2::new(half_size.x, -half_size.y),  // Bottom Right
            center + Vec2::new(half_size.x, half_size.y),   // Top Right
            center + Vec2::new(-half_size.x, half_size.y),  // Top Left
        ];

        let color = Color::linear_rgba(0.0, 255.0, 100.0, 0.9);
        // Draw lines between corners
        gizmos.line_2d(corners[0], corners[1], color);
        gizmos.line_2d(corners[1], corners[2], color);
        gizmos.line_2d(corners[2], corners[3], color);
        gizmos.line_2d(corners[3], corners[0], color);
    }
}
