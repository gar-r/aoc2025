use crate::range::Range;

pub struct Union {
    // union stores ranges sorted by min, and contains no overlap
    ranges: Vec<Range>,
}

impl Union {
    pub fn new() -> Union {
        Union { ranges: Vec::new() }
    }

    pub fn push(&mut self, r: Range) {
        self.ranges.push(r);
        self.merge();
    }

    fn merge(&mut self) {
        self.ranges.sort_by_key(|r| r.min);
        let mut merged = Vec::new();
        for r in self.ranges.clone() {
            if merged.is_empty() {
                merged.push(r);
                continue;
            }
            let prev = merged.last_mut().unwrap();
            if prev.overlaps(&r) {
                *prev = prev.merge(&r);
            } else {
                merged.push(r);
            }
        }
        self.ranges = merged
    }

    // return the total number of elements in the union
    pub fn len(&self) -> u64 {
        self.ranges.iter().fold(0u64, |sum, r| sum + r.len())
    }
}

#[cfg(test)]
mod tests {
    use crate::{range::Range, union::Union};

    #[test]
    fn test_union_push() {
        let u = &mut Union::new();
        u.push(Range::new(5, 10));
        u.push(Range::new(2, 6));
        u.push(Range::new(1, 3));
        u.push(Range::new(8, 12));
        u.push(Range::new(15, 20));

        assert_eq!(2, u.ranges.len());
        assert_eq!(Range::new(1, 12), u.ranges[0]);
        assert_eq!(Range::new(15, 20), u.ranges[1]);
    }

    #[test]
    fn test_union_len() {
        let u = &mut Union::new();
        u.push(Range::new(3, 5));
        u.push(Range::new(10, 14));
        u.push(Range::new(16, 20));
        u.push(Range::new(12, 18));

        assert_eq!(14, u.len())
    }

}