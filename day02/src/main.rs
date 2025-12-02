use util::read_input_string;

use crate::{
    id::{is_valid, is_valid_v2},
    range::{IsValidPredicate, parse_range},
};

mod id;
mod range;

fn main() {
    let data = read_input_string("input/day02-real.txt").unwrap();

    println!("part 1: {}", sum_invalids(&data, is_valid));
    println!("part 2: {}", sum_invalids(&data, is_valid_v2));
}

fn sum_invalids(data: &String, is_valid_fn: IsValidPredicate) -> i64 {
    data.split(',')
        .map(|p| p.trim())
        .map(|p| parse_range(p))
        .map(|r| r.get_invalid_ids(is_valid_fn))
        .flatten()
        .sum()
}
