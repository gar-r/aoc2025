use util::read_input_lines;

use crate::{common::Coord, part1::largest_area};

mod common;
mod part1;

fn main() {
    let path = "input/day09-real.txt";
    let coords = read_coordinates(path);

    let largest = largest_area(&coords);
    println!("part 1: {}", largest);
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
