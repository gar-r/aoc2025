use std::collections::HashSet;

/*
 * Represents a 3x3 shape with a 9-bit mask in row-major order:
 * 000000001  => top left cell
 * 100000000 => bottom right cell
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shape {
    pub mask: u16,
}

impl Shape {
    pub fn area(&self) -> usize {
        self.mask.count_ones() as usize
    }

    // Return an iterator over the _distinct_ orientations of the shape.
    pub fn orientations(&self) -> impl Iterator<Item = Shape> {
        let mut result = HashSet::new();
        let mut shape = *self;
        // 4 possible rotations combined with flip covers all possible orientations
        for _ in 0..4 {
            result.insert(shape);
            result.insert(shape.flip());
            shape = shape.rotate();
        }
        result.into_iter()
    }

    /*
     * Rotate shape 90 degrees clock-wise:
     * ABC      GAA
     * DEF  =>  HEB
     * GHI      IFC
     */
    fn rotate(&self) -> Self {
        const MAP: [usize; 9] = [2, 5, 8, 1, 4, 7, 0, 3, 6];
        Shape {
            mask: self.transform(MAP),
        }
    }

    /*
     * Flip shape across the vertical axis (mirror left-right):
     * ABC      CBA
     * DEF  =>  FED
     * GHI      IHG
     */
    fn flip(&self) -> Self {
        const MAP: [usize; 9] = [2, 1, 0, 5, 4, 3, 8, 7, 6];
        Shape {
            mask: self.transform(MAP),
        }
    }

    pub fn get_bit(&self, i: usize) -> bool {
        // shift the bit to the least-significant position, and extract the bit
        (self.mask >> i) & 1 == 1
    }

    fn transform(&self, mapping: [usize; 9]) -> u16 {
        let mut new_mask = 0u16;
        for (src, dst) in mapping.iter().enumerate() {
            if self.get_bit(src) {
                new_mask |= 1 << dst; // turn on the bit at dst
            }
        }
        new_mask
    }

    pub fn display(&self) -> String {
        let mut rows = Vec::with_capacity(3);
        for r in 0..3 {
            let mut line = String::with_capacity(3);
            for c in 0..3 {
                let idx = r * 3 + c;
                let ch = if self.get_bit(idx) { '#' } else { '.' };
                line.push(ch);
            }
            rows.push(line);
        }
        rows.join("\n")
    }
}

#[cfg(test)]
mod tests {

    use crate::shape::Shape;

    #[test]
    fn test_shape_get_bit() {
        let s = Shape { mask: 0b010101010 };
        assert!(s.get_bit(1));
        assert!(s.get_bit(3));
        assert!(s.get_bit(5));
        assert!(s.get_bit(7));

        assert!(!s.get_bit(0));
        assert!(!s.get_bit(2));
        assert!(!s.get_bit(4));
        assert!(!s.get_bit(8));
    }

    #[test]
    fn test_shape_area() {
        let s = Shape { mask: 0b111000111 };
        assert_eq!(6, s.area());
    }

    #[test]
    fn test_shape_transform() {
        let s = Shape { mask: 0b111000110 };
        // reverse the rows
        const MAP: [usize; 9] = [8, 7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(0b011000111, s.transform(MAP));
    }

    #[test]
    fn test_shape_display() {
        let s = Shape { mask: 0b011101111 };
        assert_eq!("###\n#.#\n##.", s.display());
    }

    #[test]
    fn test_shape_rotate() {
        /*  111    111
         *  100 => 101
         *  111    101
         */
        let s = Shape { mask: 0b111001111 };
        assert_eq!(0b101101111, s.rotate().mask);
    }

    #[test]
    fn test_shape_flip() {
        /*  111    111
         *  100 => 001
         *  110    011
         */
        let s = Shape { mask: 0b011001111 };
        assert_eq!(0b110100111, s.flip().mask);
    }

    #[test]
    fn test_shape_orientations() {
        /*
         *  For this shape there are 4 distinct orientations:
         *  111    001  100  010
         *  010 => 111; 111; 010
         *  010    001  100  111
         */
        let s = Shape { mask: 0b010010111 };
        let mut expected_masks = vec![0b010010111, 0b100111100, 0b001111001, 0b111010010];
        let mut masks: Vec<u16> = s.orientations().map(|o| o.mask).collect();

        masks.sort();
        expected_masks.sort();

        assert_eq!(expected_masks, masks);
    }
}
