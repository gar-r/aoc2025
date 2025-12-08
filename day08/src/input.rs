use util::read_input_lines;

use crate::coord::Coord;

pub fn parse(path: &str) -> Vec<Coord> {
    let lines = read_input_lines(path).unwrap();
    lines.iter().map(|l| parse_coord(l)).collect()
}

fn parse_coord(s: &String) -> Coord {
    let parts: Vec<&str> = s.split(",").collect();
    Coord::new(
        parts[0].parse().unwrap(),
        parts[1].parse().unwrap(),
        parts[2].parse().unwrap(),
    )
}
