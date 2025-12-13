use util::read_input_lines;

use crate::shape::Shape;

#[derive(Debug)]
pub struct Problem {
    pub area: (u8, u8),
    pub gifts: Vec<u8>,
}

pub fn parse_input(path: &str) -> (Vec<Shape>, Vec<Problem>) {
    let lines = read_input_lines(path).unwrap();
    let shapes = parse_shapes(&lines);
    let trees = parse_trees(&lines);
    (shapes, trees)
}

fn parse_trees(lines: &Vec<String>) -> Vec<Problem> {
    let tree_defs = &lines[30..];
    let mut trees = Vec::new();
    for def in tree_defs {
        let parts: Vec<&str> = def.split(":").collect();
        let a = parse_numbers(parts[0], 'x');
        trees.push(Problem {
            area: (a[0], a[1]),
            gifts: parse_numbers(parts[1].trim(), ' '),
        });
    }
    trees
}

fn parse_numbers(s: &str, separator: char) -> Vec<u8> {
    s.split(separator)
        .map(|n| n.parse::<u8>().unwrap())
        .collect()
}

fn parse_shapes(lines: &Vec<String>) -> Vec<Shape> {
    let mut shapes = Vec::new();
    //we have a total of 6 shapes
    for i in 0..6 {
        // each shape is 3 lines, surrounded by some irrelevant lines
        let start = 1 + 5 * i;
        let end = 4 + 5 * i;
        let shape = parse_shape(&lines[start..end].to_vec());
        shapes.push(shape);
    }
    shapes
}

fn parse_shape(lines: &Vec<String>) -> Shape {
    // each shape is 3x3, we store it in a 9-bit mask in row-major order
    let mut mask = 0u16;
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                let idx = r * 3 + c;
                mask |= 1 << idx;
            }
        }
    }
    Shape { mask }
}
