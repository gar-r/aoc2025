use util::Map2d;

pub fn simulate_beam(map: &mut Map2d) {
    let (sx, sy) = map.len();
    for i in 0..sx - 1 {
        for j in 0..sy {
            let c = map.get(i, j).unwrap();
            match c {
                'S' => project_beam(map, (i + 1, j)),
                '|' => project_beam(map, (i + 1, j)),
                '*' => {
                    project_beam(map, (i + 1, j - 1));
                    project_beam(map, (i + 1, j + 1));
                }
                _ => (),
            }
        }
    }
}

fn project_beam(map: &mut Map2d, pos: (usize, usize)) {
    let cell = map.get(pos.0, pos.1);
    if cell.is_none() {
        return;
    }
    match cell.unwrap() {
        '.' => map.set(pos.0, pos.1, '|'),
        '^' => map.set(pos.0, pos.1, '*'),
        _ => (),
    };
}

pub fn count_splits(map: &Map2d) -> usize {
    map.data
        .iter()
        .map(|l| l.iter().filter(|c| **c == '*').count())
        .sum()
}
