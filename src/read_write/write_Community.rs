extern crate petgraph;

use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;

use std::path::Path;
use petgraph::graph;
use std::fmt;

pub fn from_vec_of_com<'a, T: fmt::Display, U: fmt::Display>(my_vec: &Vec<Vec<petgraph::prelude::NodeIndex>>,
                                                             file_: &Path,
                                                             my_graph: &petgraph::Graph<U, T, petgraph::Undirected>){

    let out_file = File::create(file_).unwrap_or_else(|why| {
	panic!(
		"couldn't open  {}",
		file_.display(),
		)
    });

    let mut out_buff = BufWriter::with_capacity(60_000, out_file);
    for (i, community) in my_vec.iter().enumerate(){
        out_buff.write(format!(">{}\n", i).as_bytes());
        for elem in community{
             out_buff.write(format!("\t{}\n", my_graph[*elem]).as_bytes());
            }
        }

}
pub fn community_simple(){}



pub fn community_blast(){}