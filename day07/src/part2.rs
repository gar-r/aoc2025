use memoize::memoize;
use util::Map2d;

pub fn simulate_particle(map: &Map2d) -> u64 {
    let start = find_start(map).unwrap();
    simulate_particle_rec(map, start)
}

// important: we memoize the recursive function by pos(x, y),
// to avoid recalculating the count over and over again
#[memoize(Ignore: map)]
fn simulate_particle_rec(map: &Map2d, pos: (usize, usize)) -> u64 {
    let (sx, _) = map.len();
    if pos.0 == sx {
        return 1;
    }
    let cell = map.get(pos.0, pos.1);
    if cell.is_none() {
        return 0;
    }
    match cell.unwrap() {
        'S' => simulate_particle_rec(map, (pos.0 + 1, pos.1)),
        '.' => simulate_particle_rec(map, (pos.0 + 1, pos.1)),
        '^' => {
            simulate_particle_rec(map, (pos.0 + 1, pos.1 - 1))
                + simulate_particle_rec(map, (pos.0 + 1, pos.1 + 1))
        }
        _ => 0,
    }
}

fn find_start(map: &Map2d) -> Option<(usize, usize)> {
    let (sx, sy) = map.len();
    for i in 0..sx {
        for j in 0..sy {
            let c = map.get(i, j).unwrap();
            if *c == 'S' {
                return Some((i, j))
            }
        }
    }
    None
}
