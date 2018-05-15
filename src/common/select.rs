extern crate petgraph;
extern crate fnv;

use common::function;
use petgraph::Graph;
use fnv::{FnvHashSet, FnvHashMap};
use std::iter::FromIterator;
use petgraph::visit::EdgeRef;
use common;
use petgraph::graph;
use std::borrow::ToOwned;

/// create a new graph from a set of node
/// the hashmap return contain the new indice as usize and the old as usize as a string (sorry for the cast...)
/// It will keep only on edges between two nodes! to determine which one edges weight must implement "is_best" trait
///
pub fn select_from_node<U: Copy, T: common::structure::is_best<T> + Copy>(my_vec: &Vec<petgraph::graph::NodeIndex>,
                        my_graph: &petgraph::Graph<U, T, petgraph::Undirected>)
    -> (FnvHashMap<String, petgraph::graph::NodeIndex>, petgraph::Graph::<U, T, petgraph::Undirected>)
//    where
//        T: common::structure::is_best<T> + Copy,
//        U: Copy{
    {
    // Collect all node in a HashSet
    let set_node = FnvHashSet::from_iter(&my_vec);
    // the new_graph
    let mut my_new_graph = Graph::<U, T, petgraph::Undirected>::new_undirected();

	let mut my_name_to_index_hashmap: FnvHashMap<String, petgraph::graph::NodeIndex> =
        FnvHashMap::with_capacity_and_hasher(1_000_000, Default::default());

    // create a vec to add edges!
    //let mut vec_edges = Vec::with_capacity(10_000);

    // Collect all edges assert both end lie in the HashSet
    for node_index in &my_vec {
        for edge in my_graph.edges(*node_index){
            if set_node.contains(&edge.source()) && set_node.contains(&edge.target()){

                let index1 = function::get_index(&mut my_new_graph,
                                                 &mut my_name_to_index_hashmap, edge.source().index().to_string(),
                                                 my_graph[edge.source()]);

                let index2 = function::get_index(&mut my_new_graph,
                                                 &mut my_name_to_index_hashmap, edge.target().index().to_string(),
                                                 my_graph[edge.target()]);
                //vec_edges.append(edge);
                function::add_edges(&mut my_new_graph, index1, index2,
                                    *edge.weight());
            }
        }
    }
    return (my_name_to_index_hashmap, my_new_graph)
}

