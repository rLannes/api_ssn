extern crate fnv;
extern crate petgraph;

use fnv::FnvHashMap;
use fnv::FnvHashSet;

use algo::dfs::my_dfs;

pub fn cc_dfs<'a, T, U>(
    my_graph: &petgraph::Graph<U, T, petgraph::Undirected>,
) ->  Vec<Vec<petgraph::prelude::NodeIndex>>{
//FnvHashMap<u32, Vec<petgraph::graph::NodeIndex>> {
	// the hashmap already visited is updated by dsf.
    let mut already_visited = FnvHashSet::<petgraph::graph::NodeIndex>::default();
    
    //let mut comm_to_vec_node: FnvHashMap<u32, Vec<petgraph::graph::NodeIndex>> =
     //   FnvHashMap::with_capacity_and_hasher(my_graph.node_count(), Default::default());
    let mut node_community =vec![]; //2D vector self[[nodes indices com0],[node indices com1]..]

    let mut cpt = 0;

    for current_node_indice in my_graph.node_indices() {
        if already_visited.contains(&current_node_indice) {
            continue;
        }
        let current_vec: Vec<petgraph::graph::NodeIndex> =
            my_dfs(my_graph, current_node_indice.clone(), &mut already_visited);
        //comm_to_vec_node.insert(cpt, current_vec);
        node_community.push(current_vec);
        cpt = cpt + 1;
    }
   // return comm_to_vec_node;
   return node_community;
}
//Vec<Vec<petgraph::Graph::NodeIndex>>
