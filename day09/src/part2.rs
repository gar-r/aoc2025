use std::collections::HashSet;

use crate::{
    common::{Coord, Rect},
    part1::permutate_rectangles,
};

pub type EdgeSet = HashSet<Coord>;

pub fn largest_colored_area(coords: &Vec<Coord>) -> u64 {
    // calculate the area for each rect, and sort by area descending
    let mut rects: Vec<(Rect, u64)> = permutate_rectangles(coords)
        .map(|r| (r, r.area()))
        .collect();
    rects.sort_by(|a, b| a.1.cmp(&b.1));

    // create a color map containing shape edges only
    let edges = build_edge_set(coords);
    // print_edge_map(&edges, 15, 15);

    // find rect with largest area that matches the criteria
    for item in rects.iter().rev() {
        if check_rect(item.0, &edges) {
            return item.1;
        }
    }
    return 0;
}

fn check_rect(rect: Rect, edges: &EdgeSet) -> bool {
    for c in edges {
        if rect.is_inside(c) {
            return false;
        }
    }
    return true;
}

pub fn build_edge_set(coords: &Vec<Coord>) -> EdgeSet {
    let mut result = EdgeSet::new();
    for i in 1..coords.len() {
        let c1 = &coords[i - 1];
        let c2 = &coords[i];
        insert_edge(&mut result, c1, c2);
    }
    // wrap around
    insert_edge(&mut result, &coords[coords.len() - 1], &coords[0]);
    result
}

fn insert_edge(edges: &mut EdgeSet, c1: &Coord, c2: &Coord) {
    edges.insert(c1.clone());
    edges.insert(c2.clone());
    if c1.0 == c2.0 {
        // horizontal edge
        for i in (c1.1.min(c2.1) + 1)..(c1.1.max(c2.1)) {
            let coord = Coord(c1.0, i);
            edges.insert(coord);
        }
    } else if c1.1 == c2.1 {
        // vertical edge
        for i in (c1.0.min(c2.0) + 1)..(c1.0.max(c2.0)) {
            let coord = Coord(i, c1.1);
            edges.insert(coord);
        }
    }
}

#[allow(dead_code)]
pub fn print_edge_map(edges: &EdgeSet, width: usize, height: usize) {
    for i in 1..width + 1 {
        for j in 1..height + 1 {
            let row = i.try_into().unwrap();
            let col = j.try_into().unwrap();
            if edges.contains(&Coord(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!()
}
