use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

use crate::Graph;

pub fn connected_components<N, E>(g: &Graph<N, E>) -> Vec<Vec<N>>
where
    N: Eq + Hash + Clone,
    E: Clone,
{
    if !g.undirected {
        panic!("connected_components only works with undirected graphs");
    }

    let mut components = Vec::new();

    let mut visited: HashSet<N> = HashSet::new();
    for n in g.nodes() {
        if visited.contains(n) {
            continue;
        }

        // discover component with DFS
        let mut comp: Vec<N> = Vec::new();
        let mut stack: VecDeque<N> = VecDeque::new();
        stack.push_back(n.clone());

        while let Some(node) = stack.pop_back() {
            if visited.contains(&node) {
                continue;
            }

            visited.insert(node.clone());
            comp.push(node.clone());

            // add neighbors to the stack to visit in the future
            if let Some(neighbors) = g.neighbors(&node) {
                for n in neighbors.keys() {
                    if !visited.contains(n) {
                        stack.push_back(n.clone());
                    }
                }
            }
        }
        components.push(comp);
    }

    components
}

pub fn find_all_paths<N, E>(g: &Graph<N, E>, src: N, dst: N) -> Vec<Vec<N>>
where
    N: Eq + Hash + Clone,
    E: Clone,
{
    let mut result = Vec::new();

    // store the current state on the stack (current_node, current_path, current_visited)
    let mut stack: VecDeque<(N, Vec<N>, HashSet<N>)> = VecDeque::new();
    stack.push_back((src, Vec::new(), HashSet::new()));

    while let Some((node, path, visited)) = stack.pop_back() {
        if node == dst {
            result.push(path);
            continue;
        }
        for n in g.neighbors(&node).unwrap().keys() {
            if !visited.contains(n) {
                let mut new_path = path.clone();
                new_path.push(n.clone());
                let mut new_visited = visited.clone();
                new_visited.insert(n.clone());
                stack.push_back((n.clone(), new_path, new_visited));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fmt::Debug, hash::Hash};

    use crate::{
        Graph,
        graph_algo::{connected_components, find_all_paths},
    };

    #[test]
    fn test_connected_components() {
        /*
             A----B----C       E--------F
                  |
                  |                H
                  D

                        --G--
                        \---/
        */
        let mut g: Graph<char, ()> = Graph::new_undirected();
        g.add_edge('a', 'b', ());
        g.add_edge('b', 'c', ());
        g.add_edge('d', 'b', ());
        g.add_edge('f', 'e', ());
        g.add_edge('g', 'g', ()); // isolated self loop
        g.add_node('h'); // isolated node

        let components: Vec<Vec<char>> = connected_components(&g);
        assert_eq!(4, components.len());
        verify_component(&vec!['a', 'b', 'c', 'd'], &components);
        verify_component(&vec!['e', 'f'], &components);
        verify_component(&vec!['g'], &components);
        verify_component(&vec!['h'], &components);
    }

    #[track_caller]
    fn verify_component<N>(expected: &Vec<N>, components: &Vec<Vec<N>>)
    where
        N: Clone + Eq + Hash + Debug,
    {
        // verify components irrespective to the order of nodes
        assert!(components.iter().any(|c| {
            let expected_nodes: HashSet<N> = expected.iter().cloned().collect();
            let actual_nodes: HashSet<N> = c.iter().cloned().collect();
            expected_nodes.eq(&actual_nodes)
        }))
    }

    fn test_find_all_paths() {
        let mut g: Graph<char, ()> = Graph::new_directed();
        g.add_edge('s', 'e', ());
        g.add_edge('s', 'a', ());
        g.add_edge('a', 'e', ());
        g.add_edge('s', 'b', ());
        g.add_edge('b', 'd', ());
        g.add_edge('b', 'c', ());
        g.add_edge('d', 's', ());
        g.add_edge('d', 'e', ());
        g.add_edge('c', 'e', ());

        /*
         * s---->a---->e
         * s---->b---->d---->e
         * s---->b---->c---->e
         */
        let paths = find_all_paths(&g, 's', 'e');
        assert_eq!(3, paths.len());
        assert!(paths.contains(&vec!['s', 'a', 'e']));
        assert!(paths.contains(&vec!['s', 'b', 'd', 'e']));
        assert!(paths.contains(&vec!['s', 'b', 'c', 'e']));
    }
}
