use bevy::prelude::*;
mod plugin;
mod systems;

pub use plugin::HexGridPlugin;

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
        println!("Spawning new Hex entity at ({}, {}, {})", q, r, s);
        Self { q, r, s }
    }

    /// Get the neighboring hex in a given direction
    pub fn neighbor(&self, direction: usize) -> Self {
        let directions = [
            (1, 0), (1, -1), (0, -1),
            (-1, 0), (-1, 1), (0, 1)
        ];
        let (dq, dr) = directions[direction % 6];
        Self::new(self.q + dq, self.r + dr)
    }

    /// Calculate the distance between two hexes
    pub fn distance_to(&self, other: &Hex) -> i32 {
        ((self.q - other.q).abs()
            + (self.r - other.r).abs()
            + (self.s - other.s).abs()) / 2
    }

    /// Convert cube coordinates to axial (q, r)
    pub fn to_axial(&self) -> (i32, i32) {
        (self.q, self.r)
    }

    /// Convert axial coordinates back to cube representation
    pub fn from_axial(q: i32, r: i32) -> Self {
        Self::new(q, r)
    }

    pub fn world_to_hex(cursor_pos: Vec2, hex_size: f32) -> Hex {
        let q = (3.0_f32.sqrt() / 3.0 * cursor_pos.x - 1.0 / 3.0 * cursor_pos.y) / hex_size;
        let r = (2.0 / 3.0 * cursor_pos.y) / hex_size;
    
        let q_round = q.round() as i32;
        let r_round = r.round() as i32;
    
        Hex::new(q_round, r_round)
    }

    pub fn hex_to_world_position(hex: Hex, hex_size: f32) -> Vec2 {
        let x = hex_size * 3.0_f32.sqrt() * (hex.q as f32 + hex.r as f32 / 2.0);
        let y = hex_size * 1.5 * hex.r as f32;
        Vec2::new(x, y)
    }
}
