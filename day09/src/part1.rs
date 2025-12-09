use std::collections::HashSet;

use crate::common::{Coord, Rect};

pub fn largest_area(coords: &Vec<Coord>) -> u64 {
    let mut max = 0;
    for rect in permutate_rectangles(coords) {
        let area = rect.area();
        if area > max {
            max = area
        }
    }
    max
}

pub fn permutate_rectangles(coords: &Vec<Coord>) -> impl Iterator<Item = Rect> {
    let mut result = HashSet::new();
    for c1 in coords {
        for c2 in coords {
            if c1 == c2 {
                continue;
            }
            let rect = Rect(c1.clone(), c2.clone());
            result.insert(rect);
        }
    }
    result.into_iter()
}
