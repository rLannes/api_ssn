extern crate fnv;
extern crate petgraph;
use petgraph::visit::EdgeRef;
use common::structure::{NodeAttr, EdgesAttr, DicoHeader, EdgesAttrFull};

use fnv::FnvHashMap;
use fnv::FnvHashSet;
use std::iter::FromIterator;


/// if node does not exist
/// create it uptade hashmap
/// Create a node add it to the hash_table
/// index
pub fn get_index<T>(
    my_graph: &mut petgraph::Graph<NodeAttr, T, petgraph::Undirected>,
    my_map: &mut FnvHashMap<String, petgraph::graph::NodeIndex>,
    key: String,
) -> petgraph::prelude::NodeIndex {
    match my_map.get(&key) {
        Some(&number) => number, // the node exist we return the corresponding index
        None => {
            let node_index = my_graph.add_node(NodeAttr {
                name_real: key.to_string(),
            });
            my_map.insert(key, node_index);
            node_index
        }
    }
}


pub fn add_edges(my_graph: &mut petgraph::Graph<NodeAttr, EdgesAttr, petgraph::Undirected>,
	node_index1: petgraph::graph::NodeIndex,
	node_index2: petgraph::graph::NodeIndex,
	edge_atr: EdgesAttr){
	
	let test: Option<petgraph::graph::EdgeIndex> = my_graph.find_edge(node_index1, node_index2);
	
	if test.is_none(){
		//println!("edge do not exist 1");
		my_graph.add_edge(node_index1, node_index2, edge_atr); //edges do not exist we add it
		}
	
	else{
		let mut bolean = false;
		{
			let existing_edges_attributs: &EdgesAttr = &my_graph[test.unwrap()];
			if edge_atr.self_is_best(&existing_edges_attributs){
				bolean = true;
				}
			}
		if bolean{my_graph.update_edge(node_index1, node_index2, edge_atr);}
		}
	}

pub fn get_std_edges_attributs(my_vec: &[&str], header_map: &DicoHeader) -> EdgesAttr{
	let  (qcov, tcov) = compute_qcov_tcov(my_vec, header_map);
	let cov = min_f32(qcov, tcov);
	let pident =  my_vec[header_map.pid].parse::<f32>().unwrap();
	let eval = my_vec[header_map.eval].parse::<f64>().unwrap();
	let edges_properties = EdgesAttr{cov:cov, pid: pident, eval: eval};
	return edges_properties
	}


/// return the minimum from two f32 
/// i should use generic..;
///```rust
/// assert_eq!(min_f32(12.0215,0.0001), 0.0001);
///```
fn min_f32(a: f32, b: f32) -> f32 {
    if a > b {
        b
    } else {
        a
    }
}

/// Round to 2 decimal points
///
/// ```rust
/// assert_eq!(my_round(12.456), 12.46);
/// ```
fn my_round(value: f32) -> f32 {
    (value * 100.0).round().trunc() / 100.0
}

/// compute a coverage based on three &str value
/// return a rounded value at 2 decimal point
fn compute_cov(start: &str, end: &str, len: &str) -> f32 {
    let start_f = start.parse::<f32>().unwrap();
    let end_f = end.parse::<f32>().unwrap();
    let length_f = len.parse::<f32>().unwrap();
   // println!("{} {} {}", start_f, end_f, length_f);
    my_round(((end_f - start_f + 1.0) / length_f) * 100.0)
}

///
fn compute_qcov_tcov(vec: &[&str],
header_map: &DicoHeader) -> (f32, f32) {
	
        let qcov = compute_cov(vec[header_map.qstart], vec[header_map.qend], vec[header_map.qlen]);
       // println!("{}", qcov);
        let tcov = compute_cov(vec[header_map.sstart], vec[header_map.send], vec[header_map.slen]);
        //	println!("{}", tcov);
        (qcov, tcov)
}


pub fn get_full_edges_attributs(my_vec: &[&str],
	header_map: &DicoHeader, 
	node_index1: petgraph::graph::NodeIndex,
	node_index2: petgraph::graph::NodeIndex) -> EdgesAttrFull{
	
		let  (qcov, tcov) = compute_qcov_tcov(my_vec, header_map);
		let cov = min_f32(qcov, tcov);
		
		let pident =  my_vec[header_map.pid].parse::<f32>().unwrap();
		let eval = my_vec[header_map.eval].parse::<f64>().unwrap();
		
		let bitscore = my_vec[header_map.pid].parse::<f32>().unwrap();
		let qstart = my_vec[header_map.pid].parse::<i32>().unwrap(); 
		let qend = my_vec[header_map.pid].parse::<i32>().unwrap();
		let qlen = my_vec[header_map.pid].parse::<i32>().unwrap();
		let sstart = my_vec[header_map.pid].parse::<i32>().unwrap();
		let send = my_vec[header_map.pid].parse::<i32>().unwrap();
		let slen = my_vec[header_map.pid].parse::<i32>().unwrap(); 	
	
		let my_attr = EdgesAttrFull{ 
			eval: eval,
			pid: pident,
			bitscore: bitscore,
			cov: cov, // minimum coverage
			qstart: qstart,
			qend: qend,
			qlen: qlen,
			sstart: sstart,
			send: send,
			slen: slen,
			qindex: node_index1,
			sindex: node_index2};
	
	return my_attr
	}
	

pub fn add_edges_full(my_graph: &mut petgraph::Graph<NodeAttr, EdgesAttrFull, petgraph::Undirected>,
	node_index1: petgraph::graph::NodeIndex,
	node_index2: petgraph::graph::NodeIndex,
	edge_atr: EdgesAttrFull){
	
	let test: Option<petgraph::graph::EdgeIndex> = my_graph.find_edge(node_index1, node_index2);
	
	if test.is_none(){
		//println!("edge do not exist 1");
		my_graph.add_edge(node_index1, node_index2, edge_atr); //edges do not exist we add it
		}
	
	else{
		let mut bolean = false;
		{
			let existing_edges_attributs: &EdgesAttrFull = &my_graph[test.unwrap()];
			if edge_atr.self_is_best(existing_edges_attributs){
				bolean = true;
				}
			}
		if bolean{my_graph.update_edge(node_index1, node_index2, edge_atr);}
		}
	}
	

///This function takes a graph and a set of nodes_indices
/// it return a new graph -> TODO sum, mean for weitgh.
pub fn new_graph_from_nodes_index<T, U>(my_graph: &mut petgraph::Graph<T, U, petgraph::Undirected>,
	communities: Vec<Vec<petgraph::prelude::NodeIndex>>) ->
		petgraph::Graph<T, U, petgraph::Undirected>{
		
		let mut my_new_graph = petgraph::Graph::<T, U, petgraph::Undirected>::new_undirected();
		let community_number = communities.len();
		let mut community_set_vec: Vec<FnvHashSet<petgraph::prelude::NodeIndex>> = Vec::with_capacity(community_number);
		
		for (community_id, community) in communities.iter().enumerate(){
			
			let this_com_node_index_set: FnvHashSet<petgraph::prelude::NodeIndex> = community.iter().cloned().collect();
			let this_edges_set_out:FnvHashSet<petgraph::prelude::NodeIndex> = FnvHashSet::with_capacity_and_hasher(community.len(),Default::default()); // TODO bench vs try to box:: it
			let this_edges_set_in:FnvHashSet<petgraph::prelude::NodeIndex> = FnvHashSet::with_capacity_and_hasher(community.len(), Default::default());
			
			for node_index in community{
				
				for edge in my_graph.edges(*node_index){
					&my_graph[edge.id()];
					
					
					}
				
				
				}
			
			
			}
	
	return my_new_graph
	}

	
	

	
