extern crate petgraph;
extern crate fnv;
//extern crate easy_cartesians;
use fnv::FnvHashMap;
use petgraph::Graph;

//use easy_cartesians::common::d2_coordinate::{Point2d, Polar, Rectangle};
use small_cartesians_lib::module::d2;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;
use std::fmt::{self, Formatter, Display};

///svg
///specification of svg format from w3.org

pub fn write_graph_svg<'a, T, U>(my_graph: &petgraph::Graph<U, T, petgraph::Undirected>,
	position_map: &FnvHashMap<petgraph::graph::NodeIndex, d2::Vect2D>, file_: &Path) {
		  let out_file = File::create(file_).unwrap_or_else(|why| {
	panic!(
		"couldn't open  {}",
		file_.display(),
		)
    });
    
    let mut  in_file_buffer = BufWriter::with_capacity(60_000, out_file);
	in_file_buffer.write(format!("{}", "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" 
  \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n<svg viewBox=\"-300 -300 1000 1000\" 
  xmlns=\"http://www.w3.org/2000/svg\">\n").as_bytes());
  
		// iter trough array of edges
	 for edge in my_graph.raw_edges(){ 

		 let (position_1_x, position_1_y) = position_map.get(&edge.source()).unwrap().get_position();
											
		 let (position_2_x, position_2_y) = position_map.get(&edge.target()).unwrap().get_position();
											
		 in_file_buffer.write( format!( "<line x1= \"{}\" y1= \"{}\" x2= \"{}\" y2= \"{}\" stroke=\"#765373\" stroke_width=\"5\"/>\n",
		 position_1_x, position_1_y, position_2_x, position_2_y).as_bytes());
	}
		 
	for node in my_graph.node_indices(){
		let (position_node_x, position_node_y) = position_map.get(&node).unwrap().get_position();
		
		in_file_buffer.write(format!( "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"#765373\"/>\n",
		 position_node_x, position_node_y, "5.0").as_bytes());
	}
  
  write!(in_file_buffer, "{}", "</svg>\n");
		
	}

