#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Range {
    pub min: u64,
    pub max: u64,
}

impl Range {
    pub fn new(min: u64, max: u64) -> Range {
        Range { min: min, max: max }
    }

    pub fn parse(s: &str) -> Range {
        let parts: Vec<&str> = s.split("-").collect();
        let min: u64 = parts[0].parse().unwrap();
        let max: u64 = parts[1].parse().unwrap();
        Range::new(min, max)
    }

    pub fn contains(&self, n: u64) -> bool {
        self.min <= n && n <= self.max
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.contains(other.min)
            || self.contains(other.max)
            || other.contains(self.min)
            || other.contains(self.max)
    }

    // merge overlapping ranges
    // result will not be correct if used on non-overlapping ranges
    pub fn merge(&self, other: &Range) -> Range {
        Range::new(self.min.min(other.min), self.max.max(other.max))
    }

    // return the number of elements in the range
    pub fn len(&self) -> u64 {
        self.max - self.min + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::range::Range;


    #[test]
    fn test_range_contains() {
        let r = Range::new(1, 3);
        assert!(r.contains(1));
        assert!(r.contains(2));
        assert!(r.contains(3));

        assert!(!r.contains(0));
        assert!(!r.contains(4));
        assert!(!r.contains(5));
    }

    #[test]
    fn test_range_overlaps() {
        // A[---------]B
        //    C[----------]D
        {
            let r1 = Range::new(1, 3);
            let r2 = Range::new(2, 4);
            assert!(r1.overlaps(&r2));
            assert!(r2.overlaps(&r1));
        }

        //     A[---------]B
        // C[----------]D
        {
            let r1 = Range::new(2, 6);
            let r2 = Range::new(1, 4);
            assert!(r1.overlaps(&r2));
            assert!(r2.overlaps(&r1));
        }

        // A[-----]B
        //       C[----]D
        {
            let r1 = Range::new(1, 3);
            let r2 = Range::new(3, 6);
            assert!(r1.overlaps(&r2));
            assert!(r2.overlaps(&r1));
        }

        // A[-----]B
        // C[----]D
        {
            let r1 = Range::new(1, 5);
            let r2 = Range::new(1, 3);
            assert!(r1.overlaps(&r2));
            assert!(r2.overlaps(&r1));
        }

        // A[----------]B
        //    C[----]D
        {
            let r1 = Range::new(1, 5);
            let r2 = Range::new(2, 3);
            assert!(r1.overlaps(&r2));
            assert!(r2.overlaps(&r1));
        }

    }

    #[test]
    fn test_range_merge() {
        let r1 = Range::new(1, 3);
        let r2 = Range::new(2, 6);
        let merged = r1.merge(&r2);
        assert_eq!(1, merged.min);
        assert_eq!(6, merged.max);
    }

}