/// layout
extern crate easy_cartesians;
extern crate fnv;
extern crate petgraph;
extern crate rand;


use easy_cartesians::common::d2_coordinate::{Point2d, Polar, Rectangle};

use read_write::write_svg::write_graph_svg;

use petgraph::visit::EdgeRef;
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use common::function::{max_f32, min_f32};
use common::structure::{Get_weigth};

use std::cell::RefCell;
use std::f32;
use std::path::Path;

use rand::distributions::{IndependentSample, Range};



use algo::ConnectedComponent;

///
fn calc_repulsion_force(distance: f32, delta: &Point2d, rep_constante: f32) ->  Point2d{

	let force = rep_constante / distance.powf(2.0);
	let fx = (force * delta.x) / distance;
	let fy = (force * delta.y) / distance;
	return Point2d{x:fx, y:fy}
	}
/*
#[test]
fn test_repulsion(){
	
	let origin = Point2d{x: 0.0, y:0.0 };
	let one_one = Point2d{x: 1.0, y: 1.0 };
	let zero_one = Point2d{x: 0.0, y: 1.0 };
	let one_zero = Point2d{x: 1.0, y: 0.0 };
	let quarante = Point2d{x: 0.5, y: 0.5 };
	let minus_quarante = Point2d{x: 0.5, y: -0.5 };
	
	assert_eq!(calc_repulsion_force(&origin, &one_one, 1f32), Point2d{x:-1.0, y:-1.0});
	}*/

/// given two Position and a constante return a vector(force, angle)
fn calc_attraction_force(distance: f32, delta: &Point2d,
						edge_weight: f32, attraction_cst: f32, edge_size:f32) -> Point2d{
		
		let force = (attraction_cst * edge_weight) * (distance - edge_size);
		let fx = (force * delta.x) / distance;
		let fy = (force * delta.y) / distance;
		return Point2d{x:fx, y:fy}
	}
	
/*

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
	
*/	

//+ EdgeType + EdgeRef
 use petgraph::EdgeType;
pub fn layout_force_directed<'a, T: Get_weigth , U>(my_graph: &petgraph::Graph<U, T, petgraph::Undirected>,
							attract_cst: f32, rep_cst: f32, iter_num: u32, end_threshold: f32, friction: f32, edge_size: f32, delta_t:f32, MAX_DPL:f32) 
							-> (Rectangle, FnvHashMap<petgraph::graph::NodeIndex, Point2d>){
		//println!("in");
		let x: Vec<petgraph::graph::NodeIndex> = my_graph.node_indices().collect();
		
		let (cc_rectangle, cc_layout) = force_directed(my_graph, &x,
		 attract_cst, rep_cst, iter_num, end_threshold, friction, edge_size, delta_t, MAX_DPL);
	return (cc_rectangle, cc_layout);
							}
							//+ EdgeType + EdgeRef
fn force_directed<'a, T: Get_weigth  , U>(my_graph: &petgraph::Graph<U, T, petgraph::Undirected>, node_vec: &Vec<petgraph::graph::NodeIndex>,
	attract_cst: f32, rep_cst: f32, iter_num: u32, end_threshold: f32,
	 friction: f32, edge_size: f32, delta_t:f32, MAX_DPL:f32)
	 -> (Rectangle, FnvHashMap<petgraph::graph::NodeIndex, Point2d>){ 
	
	let this_rectangle = Rectangle{up_left: Point2d{x:0.0, y:0.0},
								down_rigth: Point2d{x:0.0, y:0.0}};

	// key, (position  inertie)
	let mut my_map_and_force: FnvHashMap<petgraph::graph::NodeIndex, (Point2d, Point2d)> =
	 FnvHashMap::with_capacity_and_hasher(1_000_000, Default::default());
	 
	 // uniforma ditribution random generator
	 let between = Range::new(1.0, 10.0);
	 let mut rng = rand::thread_rng();
	 
	 for node in node_vec{
		let new_x: f32 = between.ind_sample(&mut rng);
		let new_y: f32 = between.ind_sample(&mut rng);
		let mut new_position = Point2d{x: new_x, y: new_y};
		let mut speed = Point2d{x: 0.0, y: 0.0};
		//let mut interact_force = MyVector{force: 0.0, angle: 0.0};
		my_map_and_force.insert(*node, (new_position, speed));
		}
		
	let mut total_displacement:f32 = end_threshold + 50.0;
	let mut counter_iter: u32 = 0;
	
	//let mut vec_force: Vec<Point2d> = Vec::with_capacity(my_graph.node_count());
	let mut force_ = Point2d{x: 0.0, y: 0.0};
	let mut posi_ = Point2d{x:0.0, y:0.0};
	let number_of_node = node_vec.len();
	while (total_displacement > end_threshold && counter_iter < iter_num){
	
		counter_iter += 1;
		//println!("iteration: {}", counter_iter);
		let mut vec_force: Vec<Point2d> = Vec::with_capacity(my_graph.node_count());	
		counter_iter += 1;
		total_displacement = 0.0;
		let mut vec_force: Vec<Point2d> = Vec::with_capacity(my_graph.node_count());
		for elem in node_vec{
			vec_force.push(Point2d{x: 0.0, y: 0.0});
			}
			
		for indice1 in 0..number_of_node - 2{
			
			let node1_position_and_speed = my_map_and_force.get(&node_vec[indice1]).unwrap();
			let &(ref source_position, ref source_force) = node1_position_and_speed;
			
			for indice2 in indice1 + 1..number_of_node - 1{
				
				let node2_position_and_speed = my_map_and_force.get(&node_vec[indice2]).unwrap();
				let &(ref node_2_position, ref node2_force) = node2_position_and_speed;
				let delta = *node_2_position - *source_position; 
				let distance = delta.distance_from_orgine();
				if delta.x != 0.0 || delta.y != 0.0 {
					let fp = calc_repulsion_force(distance, &delta, rep_cst);
					vec_force[indice1] -= fp;
					vec_force[indice2] += fp;
					}
					
				/*else{
					//TODO
					}*/
					
				}
			}
			for edges_ in my_graph.raw_edges(){
				//let this_edges = my_graph[edges_ix];
				let source_node_index = edges_.source();
				let target_node_index = edges_.target();
				
				let node1_position_and_speed = my_map_and_force.get(&source_node_index).unwrap();
				let &(ref source_position, ref source_force) = node1_position_and_speed;
				
				let node2_position_and_speed = my_map_and_force.get(&target_node_index).unwrap();
				let &(ref node_2_position, ref node2_force) = node2_position_and_speed;
				
				let delta = *source_position - *node_2_position;
				let distance =  delta.distance_from_orgine();
				if delta.x != 0.0 || delta.y != 0.0 {
					let fp = calc_attraction_force(distance, &delta,
					 edges_.weight.get_weigth(), attract_cst, edge_size);
					vec_force[source_node_index.index()] -= fp;
					vec_force[target_node_index.index()] += fp;
				}	
		}
	
		for indice in 0..number_of_node - 1 {
			
			
			let force = vec_force[indice];
			let node1_position_and_speed = my_map_and_force.get_mut(&node_vec[indice]).unwrap();
			//println!("DEBuG node_indice: {}\n{}\nforce: {}", indice,node1_position_and_speed.0.to_string(), force.to_string());
			//let &mut(ref source_position, ref source_force) = node1_position_and_speed;
			let mut dx = delta_t * force.x;
			let mut dy = delta_t * force.y;
			let dpl = dx*dx + dy*dy;
			if dpl > MAX_DPL{
				let s = (MAX_DPL/dpl).sqrt();
				dx *= s;
				dy *= s;
				}
			
			node1_position_and_speed.0.x = node1_position_and_speed.0.x + dx;
			node1_position_and_speed.0.y = node1_position_and_speed.0.y + dy;
			total_displacement += (dx*dx + dy*dy).sqrt();
			//println!("DEBUG AFTER node_indice: {}\n{}\n{}\n\n", indice,node1_position_and_speed.0.to_string(),total_displacement);
			}
	let (this_rectangle, my_map) = clean_map(&my_map_and_force);
	let path = format!("/home/romain/iter_test{}.svg", counter_iter);
	let this_path = Path::new(&path);
	write_graph_svg(&my_graph, &my_map, &this_path);
						
		}
		
	let (this_rectangle, my_map) = clean_map(&my_map_and_force);
	let path = format!("/home/romain/iter_test{}.svg", counter_iter);
	let this_path = Path::new(&path);
	write_graph_svg(&my_graph, &my_map, &this_path);
	
	let (this_rectangle, my_map) = clean_map(&my_map_and_force);
	let path = format!("/home/romain/iter_test{}.svg", counter_iter);
	let this_path = Path::new(&path);
	write_graph_svg(&my_graph, &my_map, &this_path);
	//}
	
	let (this_rectangle, my_map) = clean_map(&my_map_and_force);
	return (this_rectangle, my_map);

	}

fn clean_map(my_map_and_force: &FnvHashMap<petgraph::graph::NodeIndex, (Point2d, Point2d)>) ->
				(Rectangle, FnvHashMap<petgraph::graph::NodeIndex, (Point2d)>) {
					
	let mut my_map: FnvHashMap<petgraph::graph::NodeIndex, (Point2d)> =
	 FnvHashMap::with_capacity_and_hasher(1_000_000, Default::default()); 
	 
	 let mut rectangle_cc = Rectangle{up_left: Point2d{x:0.0, y:0.0} ,
									down_rigth: Point2d{x:0.0, y:0.0}};

	let mut test = 0;
	 for (key, value) in my_map_and_force.iter(){
		  let &(ref  source_position, ref  source_force) = value;
		 if test == 0{
			 
			test += 1;
			
			rectangle_cc =  Rectangle{up_left: source_position.clone() ,
									down_rigth: source_position.clone()}
			 }
		 my_map.insert(*key, *source_position);
		 rectangle_cc.resize_to_include(source_position);	 
		 }
		 
	let center = rectangle_cc.get_center();

	for  value in my_map.values_mut(){
		*value = *value - center;
		}
	rectangle_cc.up_left -= center;
	rectangle_cc.down_rigth -= center;
	return (rectangle_cc, my_map)	;
					
	}
	
	/*
	 //edges_from.weight().get_weigth(), 
	//distance: f32, delta: &Point2d,
		//				edge_weight: f32, attraction_cst: f32, edge_size:f32) 
		//println!("iter num: {}, displ {}", counter_iter, total_displacement);
	
		let mut vec_force: Vec<Point2d> = Vec::with_capacity(my_graph.node_count());
		
		counter_iter += 1;
		total_displacement = 0.0;
		
		for node in node_vec{
				let mut force_ = Point2d{x: 0.0, y: 0.0};
				let mut posi_ = Point2d{x:0.0, y:0.0};
			{
			let my_value = my_map_and_force.get(node).unwrap();
			let &(ref source_position, ref source_force) = my_value;
			force_ = Point2d{x: 0.0, y: 0.0}; // will sum interaction
			posi_ = source_position.clone();
			}
			
			for node_tar in node_vec{
				if node_tar == node {
					continue;}
				
				let (ref node_position, ref node_force) = *my_map_and_force.get(node_tar).unwrap();
				let repulsion = calc_repulsion_force(&posi_, &node_position, rep_cst);
				force_  += repulsion;
			}
			
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
				let attr = calc_attraction_force(&posi_, &target_position,
					edges_from.weight().get_weigth(), attract_cst);
				println!("source {} target {}",target_position.to_string(), target_position.to_string() );
				println!("force before attr: {}", force_.to_string());
				force_ += attr;
				println!("force after attr: {}", force_.to_string());
			}
			vec_force.push(force_);
		}
		*/
		
		/*let iter_zip =  node_vec.iter().zip(vec_force.iter()); 
		
		for (i, (key, force_interaction)) in iter_zip.enumerate() {
			
			if let Some(x) = my_map_and_force.get_mut(&key) {
			
			let mut friction_v = x.1.polar_coor();
			friction_v.angle += f32::consts::PI;
			while friction_v.angle >= (2.0f32*f32::consts::PI){friction_v.angle /= 2.0f32 };
			friction_v.rayon = friction_v.rayon * friction;

	
			let acceleration = *force_interaction + friction_v.point_coor();
			x.1 = x.1 + acceleration;
			let tr = x.0 + x.1; 
			println!("node: {} accelaration: {}, old_position: {}, new_position: {}",
				i as f32, acceleration.to_string(), x.0.to_string(), tr.to_string());
			x.0  = Point2d{x: 0.0, y: 0.0} + x.1;

			total_displacement += acceleration.polar_coor().rayon;
			}			*/	
		 //}*/
