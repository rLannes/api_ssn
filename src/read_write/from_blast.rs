extern crate fnv;
extern crate petgraph;

use common::structure::{NodeAttr, EdgesAttr, DicoHeader, EdgesAttrFull};
use common::function;

use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;

use petgraph::Graph;
use fnv::FnvHashMap;



/// read a blast file
/// TODO filter value option
///return hashmap, graph <edgesAtrr (min(cov), pid, eval), Nodeattr(name_real)>
pub fn read_from_blast(header: &DicoHeader, file: &Path, threshold_values:Option<EdgesAttr>) -> 
	(
	FnvHashMap<String, petgraph::graph::NodeIndex>,
    Graph<NodeAttr, EdgesAttr, petgraph::Undirected>){

	let mut my_graph = Graph::<NodeAttr, EdgesAttr, petgraph::Undirected>::new_undirected();
        
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
		
		let this_edges_properties = function::get_std_edges_attributs(&v_line, header);
		// before going further we check if edges pass threshold
		match threshold_values{
			Some(threshold) => {if !this_edges_properties.pass_threshold(&threshold){continue}},
			None => {},
			}
		
		// get index return the graph index of a nodes , and create a node and add it to the graph if it don't exist 
		let index1 = function::get_index(&mut my_graph, &mut my_name_to_index_hashmap, v_line[header.qid].to_string());
		let index2 = function::get_index(&mut my_graph, &mut my_name_to_index_hashmap, v_line[header.qid].to_string());
		// then if edges don't exist or is of lower quality update it
		function::add_edges(&mut my_graph, index1, index2, this_edges_properties);
		}
		(my_name_to_index_hashmap, my_graph)
	}




/// return a graph with no edges attrributes but with NodeAttr(realNames)
pub fn read_from_blast_only_edges(header: &DicoHeader, file: &Path, threshold_values:Option<EdgesAttr>) -> 
	(
	FnvHashMap<String, petgraph::graph::NodeIndex>,
    Graph<NodeAttr, () , petgraph::Undirected>){

	let mut my_graph = Graph::<NodeAttr, (), petgraph::Undirected>::new_undirected();
        
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
		
		
		
		let this_edges_properties = function::get_std_edges_attributs(&v_line, header);
		// before going further we check if edges pass threshold
		match threshold_values{
			Some(threshold) => {if !this_edges_properties.pass_threshold(&threshold){continue}},
			None => {},
			}
		
		// get index return the graph index of a nodes , and create a node and add it to the graph if it don't exist 
		let index1 = function::get_index_no_weigth_on_edges(&mut my_graph, &mut my_name_to_index_hashmap, v_line[header.qid].to_string());
		let index2 = function::get_index_no_weigth_on_edges(&mut my_graph, &mut my_name_to_index_hashmap, v_line[header.qid].to_string());
		// then if edges don't exist or is of lower quality update it
		my_graph.update_edge(index1, index2, ());
		}
		(my_name_to_index_hashmap, my_graph)
	}


pub fn read_from_blast_big_edges_info(header: &DicoHeader, file: &Path, threshold_values:Option<EdgesAttr>) -> 
	(
	FnvHashMap<String, petgraph::graph::NodeIndex>,
    Graph<NodeAttr, EdgesAttrFull , petgraph::Undirected>){
		
	let mut my_graph = Graph::<NodeAttr, EdgesAttrFull, petgraph::Undirected>::new_undirected();
        
	let mut my_name_to_index_hashmap: FnvHashMap<String, petgraph::graph::NodeIndex> =
        FnvHashMap::with_capacity_and_hasher(1_000_000, Default::default());
        
	let in_file = File::open(file).unwrap_or_else(|why| {
		panic!(
			"couldn't open  {}",
			file.display(),
		)
		});
	let mut  in_file_buffer = BufReader::with_capacity(60_000, in_file); //bufering
	
	 for lines in in_file_buffer.lines(){
		
		let current_line = lines.unwrap();
		
		let v_line: Vec<_> = current_line.trim() // split line into vector
								.split_whitespace()
								.collect();
		let this_edges_properties = function::get_std_edges_attributs(&v_line, header);
		// before going further we check if edges pass threshold
		match threshold_values{
			Some(threshold) => {if !this_edges_properties.pass_threshold(&threshold){continue}},
			None => {},
			}
			
		// get index return the graph index of a nodes , and create a node and add it to the graph if it don't exist 
		let index1 = function::get_index_full_edges(&mut my_graph, &mut my_name_to_index_hashmap, v_line[header.qid].to_string());
		let index2 = function::get_index_full_edges(&mut my_graph, &mut my_name_to_index_hashmap, v_line[header.qid].to_string());
		
		
		let full = function::get_full_edges_attributs(&v_line,
		header, 
		index1,
		index2);
			
			}
		(my_name_to_index_hashmap, my_graph)
		}

