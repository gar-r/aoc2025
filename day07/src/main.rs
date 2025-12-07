use util::read_input_map2d;

use crate::{part1::{count_splits, simulate_beam}, part2::simulate_particle};

mod part1;
mod part2;

fn main() {
    const PATH: &'static str = "input/day07-real.txt";

    {
        let mut map2d = read_input_map2d(PATH).unwrap();
        simulate_beam(&mut map2d);
        println!("part 1: {}", count_splits(&map2d))
    }

    {
        let map2d = read_input_map2d(PATH).unwrap();
        let timelines = simulate_particle(&map2d);
        println!("part 2: {}", timelines);
    }
}
