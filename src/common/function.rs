extern crate fnv;
extern crate petgraph;
extern crate small_cartesians_lib;
//use petgraph::visit::EdgeRef;
use common::structure::{NodeAttr, EdgesAttr, DicoHeader, EdgesAttrFull};
use small_cartesians_lib::module::d2;
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use std::iter::FromIterator;
use common::structure::is_best;
use common;


/// if node does not exist
/// create it uptade hashmap
/// Create a node add it to the hash_table
/// index
pub fn get_index<U, T>(
    my_graph: &mut petgraph::Graph<U, T, petgraph::Undirected>,
    my_map: &mut FnvHashMap<String, petgraph::graph::NodeIndex>,
    key: String,
	weight: U
) -> petgraph::prelude::NodeIndex {
    match my_map.get(&key) {
        Some(&number) => number, // the node exist we return the corresponding index
        None => {
            let node_index = my_graph.add_node(weight);
            my_map.insert(key, node_index);
            node_index
        }
    }
}


//
pub fn get_degree<U, T>(my_graph: &petgraph::Graph<U, T, petgraph::Undirected>,
						node_index: &petgraph::graph::NodeIndex) -> u32 {
	my_graph.neighbors(*node_index).count() as u32
}


pub fn add_edges<U, T :is_best<T>>(my_graph: &mut petgraph::Graph<U, T, petgraph::Undirected>,
	node_index1: petgraph::graph::NodeIndex,
	node_index2: petgraph::graph::NodeIndex,
	edge_atr: T)
	where T: common::structure::is_best<T>{
	
	let test: Option<petgraph::graph::EdgeIndex> = my_graph.find_edge(node_index1, node_index2);
	
	if test.is_none(){
		//println!("edge do not exist 1");
		my_graph.add_edge(node_index1, node_index2, edge_atr); //edges do not exist we add it
		}
	
	else{
		let mut bolean = false;
		{
			let existing_edges_attributs = &my_graph[test.unwrap()];
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
pub fn min_f32(a: f32, b: f32) -> f32 {
    if a > b {
        b
    } else {
        a
    }
}

pub fn max_f32(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
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

pub fn remove_self_hit<U, T>(my_graph: &mut petgraph::Graph<U,T, petgraph::Undirected>){
	for node in my_graph.node_indices(){
		match my_graph.find_edge(node, node){
			None => (),
			Some(edges) =>  {my_graph.remove_edge(edges);}
		}
	}
}

/*
pub fn packingRectangle(vec_rec: &mut Vec<Rectangle>, marges: f32) -> f32 {
	
	let mut vec_size: Vec<f32> = vec![];
	//add the marge
	for rec in vec_rec.iter_mut() {
		rec.add_marges(marges);
		vec_size.push(rec.area());
		}
	vec_size.sort_by(|a, b| a.partial_cmp(b).unwrap());
		
	//  get sum(surface) of all
	 let sum_area: f32 = vec_rec.iter().fold(0.0f32, |sum, val| sum + val.area());
	 //let sum_perimeter: f32 =  vec_rec.iter().fold(0.0f32, |sum, val| sum + val.perimeter());
	 //let init_area = sum_perimeter  +
	let init_area = sum_area * 1.25;
	let _size =  init_area.sqrt().round() as f32;
	let big_rec = Rectangle{up_left: Point2d{x:0.0 , y:0.0 },
							down_rigth: Point2d{x:_size, y:_size}};
	
	let mut vec_new_position: Vec<Point2d>= vec![];
	for size in vec_rec.iter(){
		
		}
	
	
	return init_area;
	}

*/

/*
use petgraph::visit::EdgeRef;

//edge_endpoints
///This function takes a graph and a set of nodes_indices
/// it return a new graph -> TODO sum, mean for weitgh.

pub fn new_graph_from_nodes_index<T, U>(my_graph: &mut petgraph::Graph<T, U, petgraph::Undirected>,
	communities: Vec<Vec<petgraph::prelude::NodeIndex>>) ->
		petgraph::Graph<T, U, petgraph::Undirected>{
		
		let mut my_new_graph = petgraph::Graph::<T, U, petgraph::Undirected>::new_undirected();
		let community_number = communities.len();
		let mut community_set_vec: Vec<FnvHashSet<petgraph::prelude::NodeIndex>> = Vec::with_capacity(community_number);
		
		for (community_id, community) in communities.iter().enumerate(){
			
			let this_com_node_index_set: FnvHashSet<petgraph::prelude::NodeIndex> =
			 community.iter().cloned().collect();
			 
			let this_edges_set_out:FnvHashSet<petgraph::prelude::NodeIndex> =
			 FnvHashSet::with_capacity_and_hasher(community.len(),Default::default());
			  // TODO bench vs try to box:: it
			let this_edges_set_in:FnvHashSet<petgraph::prelude::NodeIndex> =
			 FnvHashSet::with_capacity_and_hasher(community.len(), Default::default());
			
			for node_index in community{
				//TODO put this part in another f() to improve clarity;
				for edge in my_graph.edges(*node_index){
					let this_edges = my_graph.edge_endpoints(edge.id()).unwrap();
					
					if this_edges.0 != *node_index {
						
						if this_com_node_index_set.contains(&this_edges.0){
							this_edges_set_in.insert(edge.id());
							}
							
						else{
							this_edges_set_out.insert(edge.id());
							}
						}
					
					else {
						
							if this_com_node_index_set.contains(&this_edges.1){
							this_edges_set_in.insert(edge.id());
							}
							
						else{
							this_edges_set_out.insert(edge.id());
							}
						}
					}
				}
			}
	
	return my_new_graph
	}
*/
	
	

	
