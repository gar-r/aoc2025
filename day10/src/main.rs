use crate::input::parse_machines;

mod input;
mod part1;
mod part2;

fn main() {
    let machines = parse_machines("input/day10-real.txt");

    let sum = machines
        .iter()
        .fold(0, |sum, m| sum + part1::find_min_button_presses(m).unwrap());
    println!("part 1: {}", sum);
}

#[derive(Debug)]
struct Machine {
    indicators: Vec<bool>,
    buttons: Vec<Button>,
    joltages: Vec<u32>,
}

#[derive(Debug)]
struct Button {
    wiring: Vec<usize>,
}
