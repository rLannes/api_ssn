extern crate fnv;
extern crate petgraph;

use fnv::FnvHashSet;



pub fn my_dfs<'a, T, U>(
    graph: &petgraph::Graph<U, T, petgraph::Undirected>,
    node: petgraph::graph::NodeIndex,
    already_visited: &mut FnvHashSet<petgraph::graph::NodeIndex>,
) -> Vec<petgraph::graph::NodeIndex> {
    let mut finded_node_index: Vec<petgraph::graph::NodeIndex> = Vec::new();
    let mut my_stack: Vec<petgraph::graph::NodeIndex> = Vec::new();

    my_stack.push(node.clone());
    already_visited.insert(node);

    while let Some(current) = my_stack.pop() {
        for neighboor in graph.neighbors(current) {
            if already_visited.contains(&neighboor) {
                continue;
            }
            my_stack.push(neighboor);
            already_visited.insert(neighboor);
        }
        finded_node_index.push(current);
    }
    finded_node_index
}
