/// layout
use common::structure::{position, MyVector};
extern crate fnv;
extern crate petgraph;

use fnv::FnvHashMap;
use fnv::FnvHashSet;


/// given two position and a constante return a vector(force, angle)
fn CalcRepulsionForce(pos_node1: &position, pos_node2: &position, rep_constante: f32) -> MyVector{
	let distance = pos_node1.distance(pos_node2); //?max
	let force = -(rep_constante/ distance.powf(2.0));
	let angle = pos_node1.bearing_angle(pos_node2);
	MyVector{force:force, angle:angle}
	
	}

/// given two position and a constante return a vector(force, angle)
fn CalcAttractionForce(pos_node1: &position, pos_node2: &position,
						edge_weight: f32, attraction_cst: f32) -> MyVector{
		let distance = pos_node1.distance(pos_node2);
		let force = attraction_cst * (distance - edge_weight);
		let angle = pos_node1.bearing_angle(pos_node2);
		MyVector{force:force, angle:angle}
	}
	
	
fn force_directed<'a, T, U>(my_graph: &petgraph::Graph<U, T, petgraph::Undirected>,
							attract_cst: f32, rep_cst: f32) -> FnvHashMap<petgraph::graph::NodeIndex, position>{
	
	
	
	}
	
