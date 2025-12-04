use util::{Map2d, read_input_map2d};

fn main() {
    let map2d = &mut read_input_map2d("input/day04-real.txt").unwrap();
    println!("part 1: {}", count_accessible_paper_rolls(map2d));
    println!("part 2: {}", count_removable_paper_rolls(map2d));
}

fn count_accessible_paper_rolls(m: &Map2d) -> usize {
    get_accessible_paper_rolls(m).len()
}

fn get_accessible_paper_rolls(m: &Map2d) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let size = m.len();
    for i in 0..size.0 {
        for j in 0..size.1 {
            if is_paper_roll(&m, i, j) && is_accessible(&m, i, j) {
                result.push((i, j));
            }
        }
    }
    result
}

fn count_removable_paper_rolls(m: &mut Map2d) -> usize {
    let mut removed: usize = 0;
    loop {
        let accessible = get_accessible_paper_rolls(m);
        let count = accessible.len();
        if accessible.len() == 0 {
            return removed;
        }
        removed += count;
        for (x, y) in accessible {
            m.set(x, y, '.'); // remove paper roll
        }
    }
}

fn is_paper_roll(m: &Map2d, x: usize, y: usize) -> bool {
    match m.get(x, y) {
        None => false,
        Some(c) => *c == '@',
    }
}

fn is_accessible(m: &Map2d, x: usize, y: usize) -> bool {
    count_adjacent_rolls(m, x, y) < 4
}

fn count_adjacent_rolls(m: &Map2d, x: usize, y: usize) -> usize {
    let adjacent = m.get_adjacent(x, y);
    let mut count = 0;
    for c in adjacent {
        if *c == '@' {
            count += 1;
        }
    }
    count
}
