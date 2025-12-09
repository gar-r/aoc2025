use core::f64;

use util::{Graph, connected_components};

use crate::{coord::Coord, input::parse};

mod coord;
mod input;

fn main() {
    let coords = parse("input/day08-real.txt");

    // pre-calculated distance between all nodes, ordered by distance
    let distances = calc_distances(&coords);

    {
        // build a graph where each node is a coord
        let mut g: Graph<Coord, ()> = Graph::new_undirected();
        coords.iter().for_each(|c| g.add_node(c.clone()));

        let limit = 1000;

        // start connecting the junction boxes up to the limit
        distances
            .iter()
            .take(limit)
            .for_each(|d| g.add_edge(d.c1.clone(), d.c2.clone(), ()));

        // find connected components, order by component size
        let mut components = connected_components(&g);
        components.sort_by(|a, b| a.len().cmp(&b.len()));

        // multiply the 3 largest component sizes
        let result = components
            .iter()
            .rev()
            .take(3)
            .fold(1, |sum, c| sum * c.len() as u64);

        println!("part 1: {}", result);
    }
    {
        // build a new graph from the coordinates
        let mut g: Graph<Coord, ()> = Graph::new_undirected();
        coords.iter().for_each(|c| g.add_node(c.clone()));

        let mut box_number = 0;

        // start adding edges, until the graph is one single component
        for i in 0..distances.len() {
            let d = &distances[i];
            g.add_edge(d.c1.clone(), d.c2.clone(), ());

            // check component count
            let components = connected_components(&g);
            if components.len() == 1 {
                box_number = i;
                break;
            }
        }

        let d = &distances[box_number];
        let result = d.c1.x * d.c2.x; // result is the x coordinates multiplied
        println!("part 2: {}", result);
    }
}

fn calc_distances(coords: &Vec<Coord>) -> Vec<Dst> {
    let len = coords.len();
    let mut result = Vec::new();
    for i in 0..len {
        for j in i + 1..len {
            let c1 = coords[i].clone();
            let c2 = coords[j].clone();
            result.push(Dst::new(c1, c2));
        }
    }
    result.sort_by(|a, b| a.dst.total_cmp(&b.dst));
    result
}

#[derive(Debug)]
struct Dst {
    c1: Coord,
    c2: Coord,
    dst: f64,
}

impl Dst {
    fn new(c1: Coord, c2: Coord) -> Dst {
        let dst = c1.distance(&c2);
        Dst {
            c1: c1,
            c2: c2,
            dst: dst,
        }
    }
}
