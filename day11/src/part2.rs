use memoize::memoize;
use util::Graph;

#[memoize(Ignore: g)]
pub fn count_all_paths(g: &Graph<String, ()>, src: String, dst: String) -> u64 {
    if src == dst {
        return 1;
    }
    g.neighbors(&src)
        .unwrap()
        .keys()
        .map(|k| count_all_paths(g, k.clone(), dst.clone()))
        .sum()
}
