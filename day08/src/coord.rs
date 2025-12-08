#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Coord {
    pub fn new(x: i64, y: i64, z: i64) -> Coord {
        Coord { x, y, z }
    }

    pub fn distance(&self, other: &Coord) -> f64 {
        f64::sqrt((
            (self.x - other.x).pow(2)
            + (self.y - other.y).pow(2)
            + (self.z - other.z).pow(2)
        ) as f64)
    }
}
