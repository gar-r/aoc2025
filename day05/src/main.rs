use util::read_input_lines;

use crate::{range::Range, union::Union};

mod range;
mod union;

fn main() {
    let (ranges, numbers) = get_input_data("input/day05-real.txt");

    // part 1: count fresh ingredients
    let count = numbers
        .iter()
        .filter(|n| ranges.iter().any(|r| r.contains(**n)))
        .count();
    println!("part 1: {}", count);

    // part 2: count all fresh ingredients
    let union = &mut Union::new();
    for r in ranges {
        union.push(r);
    }
    println!("part 2: {}", union.len());
}

fn get_input_data(path: &str) -> (Vec<Range>, Vec<u64>) {
    let lines = read_input_lines(path).unwrap();
    let mut ranges: Vec<Range> = Vec::new();
    let mut numbers: Vec<u64> = Vec::new();

    let mut parsing_ranges = true;
    for l in lines {
        if l == "" {
            parsing_ranges = false; // numbers come after an empty line
            continue;
        }
        if parsing_ranges {
            ranges.push(Range::parse(&l));
        } else {
            numbers.push(l.parse().unwrap());
        }
    }
    (ranges, numbers)
}
