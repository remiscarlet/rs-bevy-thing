#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hex {
    q: i32, // Column (Axial X)
    r: i32, // Row (Axial Z)
    s: i32, // Derived from q and r: s = -q - r
}

impl Hex {
    /// Create a new hex coordinate ensuring the constraint q + r + s = 0
    fn new(q: i32, r: i32) -> Self {
        let s = -q - r;
        Self { q, r, s }
    }

    /// Get the neighboring hex in a given direction
    fn neighbor(&self, direction: usize) -> Self {
        let directions = [
            (1, 0), (1, -1), (0, -1),
            (-1, 0), (-1, 1), (0, 1)
        ];
        let (dq, dr) = directions[direction % 6];
        Self::new(self.q + dq, self.r + dr)
    }

    /// Calculate the distance between two hexes
    fn distance_to(&self, other: &Hex) -> i32 {
        ((self.q - other.q).abs()
            + (self.r - other.r).abs()
            + (self.s - other.s).abs()) / 2
    }

    /// Convert cube coordinates to axial (q, r)
    fn to_axial(&self) -> (i32, i32) {
        (self.q, self.r)
    }

    /// Convert axial coordinates back to cube representation
    fn from_axial(q: i32, r: i32) -> Self {
        Self::new(q, r)
    }
}

pub fn demo() {
    let hex1 = Hex::new(0, 0);
    let hex2 = Hex::new(2, -1);
    
    println!("Hex1: {:?}", hex1);
    println!("Hex2: {:?}", hex2);
    println!("Distance: {}", hex1.distance_to(&hex2));

    let neighbor = hex1.neighbor(0);
    println!("Neighbor in direction 0: {:?}", neighbor);
}