use crate::{input::parse_input, part1::can_fit};

mod input;
mod part1;
mod shape;

fn main() {
    let path = "input/day12-real.txt";
    let (shapes, problems) = parse_input(path);

    let mut fits = 0;
    for p in &problems {
        match can_fit(&p, &shapes) {
            Ok(true) => fits += 1,
            Ok(false) => (),
            Err(_) => panic!("cannot solve this!"),
        }
    }

    println!("part 1: {} / {} fits", fits, &problems.len());
}
