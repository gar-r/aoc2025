use util::read_input_lines;

use crate::{common::Coord, part1::largest_area, part2::largest_colored_area};

mod common;
mod part1;
mod part2;

fn main() {
    let path = "input/day09-example.txt";
    let coords = read_coordinates(path);
    println!("part 1: {}", largest_area(&coords));
    println!("part 2: {}", largest_colored_area(&coords));
}

fn read_coordinates(path: &str) -> Vec<Coord> {
    read_input_lines(path)
        .unwrap()
        .iter()
        .map(|l| {
            let parts: Vec<&str> = l.split(",").collect();
            let x = parts[0].parse::<u64>().unwrap();
            let y = parts[1].parse::<u64>().unwrap();
            Coord(x, y)
        })
        .collect()
}
