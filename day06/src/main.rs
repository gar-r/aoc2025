use util::{read_input_lines, read_input_map2d};

use crate::worksheet::Worksheet;

mod parse;
mod parse_v2;
mod worksheet;

fn main() {
    let path = "input/day06-real.txt";

    let lines = read_input_lines(path).unwrap();
    let input = lines.iter().map(|s| s.as_str()).collect();
    let worksheet = Worksheet::parse(input);
    println!("part 1: {}", worksheet.checksum());

    let input_map = read_input_map2d(path).unwrap();
    let worksheet = Worksheet::parse_v2(&input_map);
    println!("part 2: {}", worksheet.checksum());
}
