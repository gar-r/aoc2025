#[derive(Debug)]
pub struct Range {
    start: i64,
    end: i64,
}

pub type IsValidPredicate = fn(i64) -> bool;

impl Range {
    pub fn get_invalid_ids(self, f: IsValidPredicate) -> Vec<i64> {
        let mut result = Vec::<i64>::new();
        for i in self.start..self.end + 1 {
            if !f(i) {
                result.push(i);
            }
        }
        result
    }
}

pub fn parse_range(s: &str) -> Range {
    let parts: Vec<&str> = s.split("-").collect();
    Range {
        start: must_parse(parts[0]),
        end: must_parse(parts[1]),
    }
}

fn must_parse(s: &str) -> i64 {
    s.trim().parse().unwrap()
}

#[cfg(test)]
mod tests {

    use crate::{
        id::is_valid,
        range::{Range, parse_range},
    };

    #[test]
    fn test_parse_range() {
        let r = parse_range("1698522-1698528");
        assert_eq!(1698522, r.start);
        assert_eq!(1698528, r.end);
    }

    #[test]
    fn test_parse_range_partial() {
        let r = parse_range("12-20-foo");
        assert_eq!(12, r.start);
        assert_eq!(20, r.end);
    }

    #[test]
    #[should_panic]
    fn test_parse_range_error() {
        let _ = parse_range("a-1");
    }

    #[test]
    fn test_get_invalid_ids_1() {
        let r = Range {
            start: 95,
            end: 115,
        };
        let invalids = r.get_invalid_ids(is_valid);
        assert_eq!(1, invalids.len());
        assert_eq!(99, invalids[0]);
    }

    #[test]
    fn test_get_invalid_ids_2() {
        let r = Range { start: 11, end: 22 };
        let invalids = r.get_invalid_ids(is_valid);
        assert_eq!(2, invalids.len());
        assert_eq!(11, invalids[0]);
        assert_eq!(22, invalids[1]);
    }
}
