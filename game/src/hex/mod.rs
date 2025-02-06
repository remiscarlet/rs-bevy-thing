use bevy::prelude::*;
mod plugin;
mod systems;

pub use plugin::HexGridPlugin;

#[derive(Resource, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectedHexEntity(Option<Entity>);

#[derive(Component)]
pub struct Selected;
#[derive(Component)]
pub struct Highlighted;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hex {
    q: i32, // Column (Axial X)
    r: i32, // Row (Axial Z)
    s: i32, // Derived from q and r: s = -q - r
}

impl Hex {
    /// Create a new hex coordinate ensuring the constraint q + r + s = 0
    pub fn new(q: i32, r: i32) -> Self {
        let s = -q - r;
        Self { q, r, s }
    }

    /// Get the neighboring hex in a given direction
    pub fn neighbor(&self, direction: usize) -> Self {
        let directions = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];
        let (dq, dr) = directions[direction % 6];
        Self::new(self.q + dq, self.r + dr)
    }

    /// Calculate the distance between two hexes
    pub fn distance_to(&self, other: &Hex) -> u32 {
        ((self.q - other.q).unsigned_abs()
            + (self.r - other.r).unsigned_abs()
            + (self.s - other.s).unsigned_abs())
            / 2
    }

    /// Convert cube coordinates to axial (q, r)
    pub fn to_axial(&self) -> (i32, i32) {
        (self.q, self.r)
    }

    /// Convert axial coordinates back to cube representation
    pub fn from_axial(q: i32, r: i32) -> Self {
        Self::new(q, r)
    }

    pub fn world_to_hex(world_pos: Vec2, hex_size: f32) -> Hex {
        let q = (3.0_f32.sqrt() / 3.0 * world_pos.x - 1.0 / 3.0 * world_pos.y) / hex_size;
        let r = (2.0 / 3.0 * world_pos.y) / hex_size;

        let q_round = q.round() as i32;
        let r_round = r.round() as i32;

        Hex::new(q_round, r_round)
    }

    /**
     * - Key intuition:
     *   - Assume pointy top hexagons
     *   - `size` is defined as the radius of the circle enclosing a hexagon (outer circle)
     *   - 1) the width of the hexagon, asset, and horizontal spacing = `sqrt(3) * size`
     *   - 2) The HEIGHT of the hexagon, asset = `2 * size`
     *   - 3) The VERTICAL SPACING between two hexagons = `3/2 * size`
     */
    /**
     * Source: https://www.redblobgames.com/grids/hexagons/#hex-to-pixel-axial
     */
    pub fn hex_to_world_position(hex: Hex, hex_size: f32) -> Vec2 {
        let x = hex_size * 3.0_f32.sqrt() * (hex.q as f32 + hex.r as f32 / 2.0);
        let y = hex_size * 1.5 * hex.r as f32;
        Vec2::new(x, y)
    }
    // pub fn hex_to_world_position(hex: Hex, hex_size: f32) -> Vec2 {
    //     let x = hex_size * 3.0_f32.sqrt() / 2.0 * hex.q as f32;
    //     let y = hex_size * 1.5 * hex.r as f32;
    //     Vec2::new(x, y)
    // }
}
