use util::{Graph, read_input_lines};

pub fn build_input(path: &str) -> Graph<String, ()> {
    let lines = read_input_lines(path).unwrap();
    build_graph(lines)
}

fn build_graph(lines: Vec<String>) -> Graph<String, ()> {
    let mut g = Graph::new_directed();
    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let src = parts[0].trim();
        let dst: Vec<&str> = parts[1].trim().split(' ').collect();
        for d in dst {
            g.add_edge(src.to_owned(), d.to_owned(), ());
        }
    }
    g
}

#[cfg(test)]
mod tests {
    use crate::input::build_graph;

    #[test]
    fn test_build_graph() {
        let input = vec![
            String::from("a: b c"),
            String::from("b: d"),
            String::from("c: e f"),
        ];
        let g = build_graph(input);

        // a-->b, a-->c
        let neighbors = g.neighbors(&"a".to_owned()).unwrap();
        assert!(neighbors.contains_key("b"));
        assert!(neighbors.contains_key("c"));

        // b-->d
        let neighbors = g.neighbors(&"b".to_owned()).unwrap();
        assert!(neighbors.contains_key("d"));

        // c-->e, c-->f
        let neighbors = g.neighbors(&"c".to_owned()).unwrap();
        assert!(neighbors.contains_key("e"));
        assert!(neighbors.contains_key("f"));

        // rest of the nodes have no neighbors
        assert!(g.neighbors(&"d".to_owned()).unwrap().is_empty());
        assert!(g.neighbors(&"e".to_owned()).unwrap().is_empty());
        assert!(g.neighbors(&"f".to_owned()).unwrap().is_empty());
    }
}
