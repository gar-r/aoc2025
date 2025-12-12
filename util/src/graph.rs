use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Default)]
pub struct Graph<N, E> {
    adj: HashMap<N, HashMap<N, E>>,
    pub undirected: bool,
}

impl<N, E> Graph<N, E>
where
    N: Eq + Hash + Clone,
    E: Clone,
{
    pub fn new_undirected() -> Self {
        Graph::new(true)
    }

    pub fn new_directed() -> Self {
        Graph::new(false)
    }

    fn new(undirected: bool) -> Self {
        Self {
            adj: HashMap::new(),
            undirected,
        }
    }

    pub fn add_node(&mut self, node: N) {
        if !self.adj.contains_key(&node) {
            self.adj.insert(node, HashMap::new());
        }
    }

    pub fn add_edge(&mut self, src: N, dst: N, weight: E) {
        self.add_node(src.clone());
        self.add_node(dst.clone());
        self.adj
            .get_mut(&src)
            .unwrap()
            .insert(dst.clone(), weight.clone());
        if self.undirected {
            self.adj
                .get_mut(&dst)
                .unwrap()
                .insert(src.clone(), weight.clone());
        }
    }

    pub fn delete_edge(&mut self, src: &N, dst: &N) -> bool {
        let mut removed = false;
        if let Some(map) = self.adj.get_mut(src) {
            removed = map.remove(dst).is_some();
        }
        if self.undirected {
            if let Some(map) = self.adj.get_mut(dst) {
                removed |= map.remove(src).is_some();
            }
        }
        removed
    }

    pub fn neighbors(&self, node: &N) -> Option<&HashMap<N, E>> {
        self.adj.get(node)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &N> {
        self.adj.keys()
    }
}

#[cfg(test)]
mod tests {
    use crate::Graph;

    #[test]
    fn test_graph_new() {
        let g: Graph<usize, ()> = Graph::new(true);
        assert_eq!(0, g.adj.len());
        assert_eq!(true, g.undirected);
    }

    #[test]
    fn test_graph_add_node() {
        let mut g: Graph<usize, ()> = Graph::new(true);
        g.add_node(13);
        g.add_node(42);
        assert_eq!(2, g.adj.len());
        assert!(g.adj.contains_key(&13));
        assert!(g.adj.contains_key(&42));
    }

    #[test]
    fn test_graph_add_edge() {
        let mut g: Graph<usize, i8> = Graph::new(true);
        g.add_edge(13, 42, 5);
        assert_eq!(5, *g.adj.get(&13).unwrap().get(&42).unwrap());
        assert_eq!(5, *g.adj.get(&42).unwrap().get(&13).unwrap());
    }

    #[test]
    fn test_graph_neighbors() {
        let mut g: Graph<usize, i8> = Graph::new(true);
        g.add_edge(13, 42, 5);
        let n = g.neighbors(&13);
        assert!(n.is_some());
        assert_eq!(1, n.unwrap().len());
        assert_eq!(5, *n.unwrap().get(&42).unwrap());

        // no such node
        let n = g.neighbors(&20);
        assert!(n.is_none());
    }

    #[test]
    fn test_graph_neighbors_undirected() {
        let mut g: Graph<usize, i8> = Graph::new(false);
        g.add_edge(10, 15, 1);

        let n = g.neighbors(&10);
        assert_eq!(1, n.unwrap().len());
        assert!(n.unwrap().contains_key(&15));

        let n = g.neighbors(&15);
        assert_eq!(0, n.unwrap().len());
    }

    #[test]
    fn test_graph_delete_edge() {
        let mut g: Graph<usize, i8> = Graph::new(true);
        g.add_edge(13, 42, 5);
        let removed = g.delete_edge(&13, &42);
        assert!(removed);
        assert_eq!(0, g.adj.get(&13).unwrap().len());
        assert_eq!(0, g.adj.get(&42).unwrap().len());
    }

    #[test]
    fn test_graph_nodes() {
        let mut g: Graph<usize, ()> = Graph::new(true);
        g.add_node(1);
        g.add_node(2);
        g.add_node(3);
        assert_eq!(3, g.adj.len());
        assert!(g.adj.contains_key(&1));
        assert!(g.adj.contains_key(&2));
        assert!(g.adj.contains_key(&3));
    }
}
