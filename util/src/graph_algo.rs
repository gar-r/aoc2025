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
            if let Some(neighbors) = g.neightbors(&node) {
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

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fmt::Debug, hash::Hash};

    use crate::{Graph, graph_algo::connected_components};

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
}
