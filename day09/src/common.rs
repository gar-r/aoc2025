use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(pub u64, pub u64);

#[derive(Debug, Eq, Clone, Copy)]
pub struct Rect(pub Coord, pub Coord);

impl Rect {
    pub fn area(&self) -> u64 {
        self.width() * self.height()
    }
    pub fn is_inside(&self, coord: &Coord) -> bool {
        let c1 = self.0;
        let c2 = self.1;
        coord.0 > c1.0.min(c2.0)
            && coord.0 < c1.0.max(c2.0)
            && coord.1 > c1.1.min(c2.1)
            && coord.1 < c1.1.max(c2.1)
    }
    pub fn width(&self) -> u64 {
        1 + (&self.0).0.abs_diff((&self.1).0)
    }
    pub fn height(&self) -> u64 {
        1 + (&self.0).1.abs_diff((&self.1).1)
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        // consider (a,b) and (b, a) the same Rect
        self.0 == other.0 && self.1 == other.1 || self.1 == other.0 && self.0 == other.1
    }
}

impl Hash for Rect {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // hash the endpoints in sorted order so (a,b) and (b,a) hash the same
        let c1 = &self.0;
        let c2 = &self.1;
        if c1.0 < c2.0 {
            c1.hash(state);
            c2.hash(state);
        } else if c1.0 > c2.0 {
            c2.hash(state);
            c1.hash(state);
        } else {
            // x coordinates are equal, sort by y coords
            if c1.1 <= c2.1 {
                c1.hash(state);
                c2.hash(state);
            } else {
                c2.hash(state);
                c1.hash(state);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash};

    use crate::common::{Coord, Rect};

    #[test]
    fn test_rectange_partial_eq() {
        let r1 = Rect(Coord(2, 2), Coord(5, 3));
        let r2 = Rect(Coord(5, 3), Coord(2, 2));
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_rectangle_hash() {
        let r1 = Rect(Coord(2, 2), Coord(5, 3));
        let r2 = Rect(Coord(5, 3), Coord(2, 2));
        let mut hasher = DefaultHasher::new();
        assert_eq!(r1.hash(&mut hasher), r2.hash(&mut hasher));
    }

    #[test]
    fn test_rectange_area() {
        let r = Rect(Coord(9, 7), Coord(2, 5));
        assert_eq!(24, r.area())
    }

    #[test]
    fn test_rectangle_is_inside() {
        let r = Rect(Coord(7, 7), Coord(2, 3));
        assert!(!r.is_inside(&Coord(1, 1)));
        assert!(!r.is_inside(&Coord(2, 3)));
        assert!(!r.is_inside(&Coord(2, 4)));
        assert!(!r.is_inside(&Coord(7, 7)));
        assert!(!r.is_inside(&Coord(8, 8)));

        assert!(r.is_inside(&Coord(3, 4)));
        assert!(r.is_inside(&Coord(6, 6)));
        assert!(r.is_inside(&Coord(5, 5)));
    }
}
