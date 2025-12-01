use util::read_input_lines;

use crate::dial::Dial;

mod dial;
mod instr;

fn main() {
    let lines = read_input_lines("input/day01-real.txt").unwrap();
    let dial = &mut Dial { val: 50, of: 0 };
    let mut count = 0;
    let mut overflows = 0;
    for l in lines {
        let (dir, val) = instr::parse(l.as_str());
        dial.rotate(dir, val);
        if dial.val == 0 {
            count += 1;
        }
        overflows += dial.of;
    }
    println!("part 1: {}", count);
    println!("part 2: {}", overflows);
}
