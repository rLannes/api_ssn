/// layout
use common::structure::{Position, MyVector, Rectangle, Get_weigth};
extern crate fnv;
extern crate petgraph;
extern crate rand;
use petgraph::visit::EdgeRef;
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::cell::RefCell;
//use rand::{weak_rng, Rng};
//use rand::distributions::Uniform;


use rand::distributions::{IndependentSample, Range};



use algo::ConnectedComponent;

/// given two Position and a constante return a vector(force, angle)
fn calc_repulsion_force(pos_node1: &Position, pos_node2: &Position, rep_constante: f32) -> MyVector{
	let distance = pos_node1.distance(pos_node2); //?max
	let force = -(rep_constante/ distance.powf(2.0));
	let angle = pos_node1.bearing_angle(pos_node2);
	MyVector{force:force, angle:angle}
	
	}

/// given two Position and a constante return a vector(force, angle)
fn calc_attraction_force(pos_node1: &Position, pos_node2: &Position,
						edge_weight: f32, attraction_cst: f32) -> MyVector{
		let distance = pos_node1.distance(pos_node2);
		let force = attraction_cst * (distance - edge_weight);
		let angle = pos_node1.bearing_angle(pos_node2);
		MyVector{force:force, angle:angle}
	}
	
fn layout_force_directed<'a, T: Get_weigth, U>(my_graph: &petgraph::Graph<U, T, petgraph::Undirected>,
							attract_cst: f32, rep_cst: f32, iter_num: u32, end_threshold: f32){
								
	let my_vec = ConnectedComponent::cc_dfs(&my_graph);
	let mut singleton: Vec<Vec<petgraph::prelude::NodeIndex>> = vec![];
	let mut doublon: Vec<Vec<petgraph::prelude::NodeIndex>> = vec![];
	
	
	for composante in &my_vec{
		
		if composante.len() == 1 {
			singleton.push(composante.clone())
			}
			
		else if composante.len() == 2{
			doublon.push(composante.clone())
			}
			
		else {
			let (cc_rectangle, cc_layout) = force_directed(my_graph, composante, attract_cst, rep_cst, iter_num, end_threshold);
			}
		}
	
	}

fn force_directed<'a, T: Get_weigth, U>(my_graph: &petgraph::Graph<U, T, petgraph::Undirected>, node_vec: &Vec<petgraph::graph::NodeIndex>,
							attract_cst: f32, rep_cst: f32, iter_num: u32, end_threshold: f32)
							 -> (Rectangle, FnvHashMap<petgraph::graph::NodeIndex, Position>){
								 
	let this_rectangle = Rectangle{position_up_left: Position{x:0.0, y:0.0}, position_down_rigth: Position{x:0.0, y:0.0}};
	// init the hash_map
	let mut my_map_and_force: FnvHashMap<petgraph::graph::NodeIndex, RefCell<(Position, MyVector)>> =
	 FnvHashMap::with_capacity_and_hasher(1_000_000, Default::default()); 
	
	//init with random node Position
	 let between = Range::new(0.1, 10.0);
	 let mut rng = rand::thread_rng();
	 
	for node in node_vec{
		let new_x: f32 = between.ind_sample(&mut rng);
		let new_y: f32 = between.ind_sample(&mut rng);
		let mut new_position = Position{x: new_x, y: new_y};
		let mut f_vec = MyVector{force: 0.0, angle: 0.0};
		my_map_and_force.insert(*node, RefCell::new((new_position, f_vec)));
	}	
	
	let mut total_displacement:f32 = 100.0 * node_vec.len() as f32;
	let mut counter_iter: u32 = 0;
	
	while (total_displacement > end_threshold && counter_iter < iter_num){
		counter_iter += 1;
		total_displacement = 0.0;
		
		for node in node_vec{
			
			let mut force = MyVector{force: 0.0, angle: 0.0};
			
			//let my_value = match my_map_and_force.entry(*node){
			//	  Vacant(entry) => panic!("key_not found"),
			//	 Occupied(entry) => entry.into_mut(),};
			let my_value = my_map_and_force.get(node).unwrap();
			let mut borrowed_tuple = my_value.borrow_mut();
			let (ref mut  source_position, ref mut source_force) = *borrowed_tuple;
			//let &mut(ref mut  source_position, ref mut source_force) = my_value.borrow();
			//let &mut(ref mut  source_position, ref mut source_force) = my_map_and_force.get_mut(&node).unwrap();
			//let &mut(ref mut  source_position, ref mut source_force)
			force = *source_force ;
			
			for node_tar in node_vec{
				let (ref node_position, ref node_force) = *my_map_and_force.get(node_tar).unwrap().borrow();
				//let (ref node_position, ref node_force) = *my_map_and_force.get(&node_tar).unwrap();
				let repulsion = calc_repulsion_force(&source_position, &node_position, rep_cst);
				force =  force.somme(&repulsion);
			}
				
			for edges_from in my_graph.edges(*node){
				let edges_ = &my_graph[edges_from.id()];
				
				//pos_node1: &Position, pos_node2: &Position,
				//		edge_weight: f32, attraction_cst: f32
				let mut my_node  =  node.clone();
				
				if edges_from.source() != *node{
					my_node = edges_from.source();
					}
					
				else{
					my_node = edges_from.target();
					} 
					
				let (ref target_position, ref target_force) = *my_map_and_force.get(&my_node).unwrap().borrow();
				//let (ref target_position, ref target_force) = *my_map_and_force.get(&my_node).unwrap();
				
				let attr = calc_attraction_force(&source_position, &target_position,
				 edges_from.weight().get_weigth(), attract_cst);
				 force =  force.somme(&attr);
			}
			*source_force = force;
			/*if let Some(x) = my_map_and_force.get_mut(&node) {
				let &mut(ref mut r, ref mut y) = x;
				*y =  force;*/
			//}
		}
		// change Position of nodes usingt the vector computed value
		for this_ref_cell in my_map_and_force.values(){
			let (ref mut source_position, ref mut source_force) = *this_ref_cell.borrow_mut();
			source_position.add_vector(source_force);
			total_displacement += source_force.force;
			
			}
		
	}
		
		// todo recopy; removing the REfCell making Rectangle kepping only Position
		let mut my_map: FnvHashMap<petgraph::graph::NodeIndex, (Position)> =
	 FnvHashMap::with_capacity_and_hasher(1_000_000, Default::default()); 
	 let mut rectangle_cc = Rectangle{position_up_left: Position{x:0.0, y:0.0} , position_down_rigth: Position{x:0.0, y:0.0}};
	 for (key, value) in my_map_and_force.drain(){
		 let (ref  source_position, ref  source_force) = *value.borrow();
		 my_map.insert(key, *source_position);
		 rectangle_cc.update_if_bigger(source_position);
		 
		 }
	
	return (this_rectangle, my_map);
	}

//pos_node1: &Position, pos_node2: &Position, rep_constante: f32

