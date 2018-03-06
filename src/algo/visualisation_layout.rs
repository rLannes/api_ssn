/// layout
use common::structure::{Get_weigth};
use common::geometry::{Position, Rectangle, MyVector};
use easy_cartesians::
use read_write::write_svg::write_graph_svg;
extern crate fnv;
extern crate petgraph;
extern crate rand;
use petgraph::visit::EdgeRef;
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use common::function::{max_f32, min_f32};


use std::cell::RefCell;
//use rand::{weak_rng, Rng};
//use rand::distributions::Uniform;
use std::f32;
use std::path::Path;

use rand::distributions::{IndependentSample, Range};



use algo::ConnectedComponent;

/// given two Position and a constante return a vector(force, angle)
fn calc_repulsion_force(pos_node1: &Position, pos_node2: &Position, rep_constante: f32) -> MyVector{
	
	let mut  distance = pos_node1.distance(pos_node2); //?max
	distance = distance;
	
	let force = rep_constante / distance.powf(2.0);
	let  angle = pos_node1.sub_position(pos_node2).to_polar().angle;
		println!("repulsion result angle: {}, force {}", angle, force);
		pos_node1.print();
		pos_node2.print();
	MyVector{force:force, angle:angle}
	
	}

/// given two Position and a constante return a vector(force, angle)
fn calc_attraction_force(pos_node1: &Position, pos_node2: &Position,
						edge_weight: f32, attraction_cst: f32) -> MyVector{
		let distance = pos_node1.distance(pos_node2);
		//let force = (attraction_cst * edge_weight) / distance;
		let force = (attraction_cst * edge_weight) * distance;
		let angle = pos_node2.sub_position(pos_node1).to_polar().angle;
		
		println!("attraction result angle: {}, force {}", angle, force);
		pos_node1.print();
		pos_node2.print();
		MyVector{force:force, angle:angle}
	}
	
pub fn layout_force_directed_multi_cc<'a, T: Get_weigth, U>(my_graph: &petgraph::Graph<U, T, petgraph::Undirected>,
							attract_cst: f32, rep_cst: f32, iter_num: u32, end_threshold: f32, friction: f32){
								
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
			let (cc_rectangle, cc_layout) = force_directed(my_graph, composante,
			 attract_cst, rep_cst, iter_num, end_threshold, friction);
			}
		}
	
	}
	
	
pub fn layout_force_directed<'a, T: Get_weigth, U>(my_graph: &petgraph::Graph<U, T, petgraph::Undirected>,
							attract_cst: f32, rep_cst: f32, iter_num: u32, end_threshold: f32, friction: f32) 
							-> (Rectangle, FnvHashMap<petgraph::graph::NodeIndex, Position>){
		//println!("in");
		let x: Vec<petgraph::graph::NodeIndex> = my_graph.node_indices().collect();
		
		let (cc_rectangle, cc_layout) = force_directed(my_graph, &x,
		 attract_cst, rep_cst, iter_num, end_threshold, friction);
	return (cc_rectangle, cc_layout);
							}
							
fn force_directed<'a, T: Get_weigth, U>(my_graph: &petgraph::Graph<U, T, petgraph::Undirected>, node_vec: &Vec<petgraph::graph::NodeIndex>,
	attract_cst: f32, rep_cst: f32, iter_num: u32, end_threshold: f32, friction: f32)
	 -> (Rectangle, FnvHashMap<petgraph::graph::NodeIndex, Position>){ 
		 
	let this_rectangle = Rectangle{position_up_left: Position{x:0.0, y:0.0},
								position_down_rigth: Position{x:0.0, y:0.0}};
								
	let mut my_map_and_force: FnvHashMap<petgraph::graph::NodeIndex, (Position, MyVector)> =
	 FnvHashMap::with_capacity_and_hasher(1_000_000, Default::default());
	 
	 // uniforma ditribution random generator
	 let between = Range::new(0.1, 10.0);
	 let mut rng = rand::thread_rng();
	 
	 for node in node_vec{
		let new_x: f32 = between.ind_sample(&mut rng);
		let new_y: f32 = between.ind_sample(&mut rng);
		let mut new_position = Position{x: new_x, y: new_y};
		let mut speed = MyVector{force: 0.0, angle: 0.0};
		//let mut interact_force = MyVector{force: 0.0, angle: 0.0};
		my_map_and_force.insert(*node, (new_position, speed));
		}
		
	let mut total_displacement:f32 = end_threshold + 50.0;
	let mut counter_iter: u32 = 0;
	
	let mut vec_force: Vec<MyVector> = Vec::with_capacity(my_graph.node_count());
	let mut force_ = MyVector{force: 0.0, angle: 0.0};
	let mut posi_ = Position{x:0.0, y:0.0};
	
	while (total_displacement > end_threshold && counter_iter < iter_num){
		//println!("iter num: {}, displ {}", counter_iter, total_displacement);
		
		let mut vec_force: Vec<MyVector> = Vec::with_capacity(my_graph.node_count());
		
		counter_iter += 1;
		total_displacement = 0.0;
		
		for node in node_vec{
			
			{
			let my_value = my_map_and_force.get(node).unwrap();
			let &(ref source_position, ref source_force) = my_value;
			//let mut borrowed_tuple = my_value.borrow_mut();			
			//let (ref   source_position, ref source_force) = *borrowed_tuple;
			force_ = MyVector{force: 0.0, angle: 0.0}; // will sum interaction
			posi_ = source_position.clone();
			}
			
			for node_tar in node_vec{
				if node_tar == node {
					continue;}
				
				let (ref node_position, ref node_force) = *my_map_and_force.get(node_tar).unwrap();
				let repulsion = calc_repulsion_force(&posi_, &node_position, rep_cst);
				force_ =  force_.somme(&repulsion);
				//force_.print();
				//println!("end rep");
			}
			
			force_.print();
			for edges_from in my_graph.edges(*node){
				
				let edges_ = &my_graph[edges_from.id()];
				
				let mut my_node  =  node.clone();
				
				if edges_from.source() != *node{
					my_node = edges_from.source();
					}
					
				else{
					my_node = edges_from.target();
					} 

				let (ref target_position, ref target_force) = *my_map_and_force.get(&my_node).unwrap();
				//println!("attract debug");
				//target_position.print();
				//posi_.print();
				let attr = calc_attraction_force(&posi_, &target_position,
					edges_from.weight().get_weigth(), attract_cst);
				//attr.print();
				//println!("attract end");
			
				force_ =  force_.somme(&attr);
			}
			force_.print();
			vec_force.push(force_);
		}
		
		let iter_zip =  node_vec.iter().zip(vec_force.iter()); 
		
		for (i, (key, force_interaction)) in iter_zip.enumerate() {
			
			if let Some(x) = my_map_and_force.get_mut(&key) {
			
			let mut friction_angle = x.1.angle + f32::consts::PI;
			while friction_angle >= (2.0f32*f32::consts::PI){friction_angle /= 2.0f32 };
			let friction_vector = MyVector{force: x.1.force * friction,
				 angle: friction_angle};
			println!("test friction ");
			force_interaction.print();
			x.1.print();
			friction_vector.print();
			
			let acceleration = force_interaction.somme(&friction_vector);
			x.1 = x.1.somme(&acceleration);
			x.0.add_vector(&x.1);	
			total_displacement += x.1.force;
			
			}				
		 }
	let (this_rectangle, my_map) = clean_map(&my_map_and_force);
	let path = format!("/home/romain/iter_test{}.svg", counter_iter);
	let this_path = Path::new(&path);
	write_graph_svg(&my_graph, &my_map, &this_path);
	}
	let (this_rectangle, my_map) = clean_map(&my_map_and_force);
	return (this_rectangle, my_map);

	}

fn clean_map(my_map_and_force: &FnvHashMap<petgraph::graph::NodeIndex, (Position, MyVector)>) ->
				(Rectangle, FnvHashMap<petgraph::graph::NodeIndex, (Position)>) {
					
	let mut my_map: FnvHashMap<petgraph::graph::NodeIndex, (Position)> =
	 FnvHashMap::with_capacity_and_hasher(1_000_000, Default::default()); 
	 
	 let mut rectangle_cc = Rectangle{position_up_left: Position{x:0.0, y:0.0} ,
									position_down_rigth: Position{x:0.0, y:0.0}};

	let mut test = 0;
	 for (key, value) in my_map_and_force.iter(){
		  let &(ref  source_position, ref  source_force) = value;
		 if test == 0{
			 
			test += 1;
			
			rectangle_cc =  Rectangle{position_up_left: source_position.clone() ,
									position_down_rigth: source_position.clone()}
			 }
		 my_map.insert(*key, *source_position);
		 rectangle_cc.update_if_bigger(source_position);	 
		 }
		 
	let center = rectangle_cc.get_center();

	for  value in my_map.values_mut(){
		value.substract(&center);
		}
	rectangle_cc.center(&center);

	return (rectangle_cc, my_map)	;
					
	}
	
