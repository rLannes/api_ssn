extern crate fnv;
extern crate petgraph;

use common::structure::{NodeAttr, EdgesAttr, LigthEdges};
use common::function;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;
use petgraph::Graph;
use fnv::FnvHashMap;

pub fn read_from_edges(file: &Path, threshold_values:Option<EdgesAttr>) -> 
	(
	FnvHashMap<String, petgraph::graph::NodeIndex>,
    Graph<NodeAttr, LigthEdges, petgraph::Undirected>){
		
	let mut my_graph = Graph::<NodeAttr, LigthEdges, petgraph::Undirected>::new_undirected();
        
	let mut my_name_to_index_hashmap: FnvHashMap<String, petgraph::graph::NodeIndex> =
        FnvHashMap::with_capacity_and_hasher(1_000_000, Default::default());
        
   let in_file = File::open(file).unwrap_or_else(|why| {
	panic!(
		"couldn't open  {}",
		file.display(),
	)
    });
    
    //ToDO clean this sepearte in other foo()
    let mut  in_file_buffer = BufReader::with_capacity(60_000, in_file); //bufering
    
    for lines in in_file_buffer.lines(){
		
		let current_line = lines.unwrap();
		let v_line: Vec<_> = current_line.trim() // split line into vector
								.split_whitespace()
								.collect();
		let mut default_weight: LigthEdges;
		if v_line.len() == 2{
			default_weight = LigthEdges{weight:1.0};
			}
			
		else{
			default_weight = LigthEdges{weight:v_line[3].parse::<f32>().unwrap()};
			}
			
		let index1 = function::get_index(&mut my_graph, &mut my_name_to_index_hashmap, v_line[0].to_string());
		let index2 = function::get_index(&mut my_graph, &mut my_name_to_index_hashmap, v_line[1].to_string());
		my_graph.update_edge(index1, index2, default_weight);
		
		}
		(my_name_to_index_hashmap, my_graph)
		
	}