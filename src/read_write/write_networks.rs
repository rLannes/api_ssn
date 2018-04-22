extern crate fnv;
extern crate petgraph;

use common::structure::{NodeAttr, EdgesAttr, LigthEdges, EdgesAttrFull};
use common::function;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;
use petgraph::Graph;
use fnv::FnvHashMap;
use petgraph::visit::{ NodeIndexable,NodeCount,IntoNodeIdentifiers};
use petgraph::graph::{Node, Edge};
use std::fmt;
//<N: fmt::Display, T: fmt::Display>
/// default function that will display the network
/// it is in a tsv format
/// for each edges it print the node attributs first then the edges attributes
pub fn write_networks<N: fmt::Display, T: fmt::Display>(file: &Path,
                         my_graph: petgraph::Graph<Node<N>, Edge<T>>) -> ()
{
    let out_file = File::create(file).unwrap_or_else(|why| {
	    panic!(
		    "couldn't open  {}",
		    file.display(),
		    )
        });

        let mut  out_file_buffer = BufWriter::with_capacity(60_000, out_file);

        for edges_ in my_graph.edge_indices() {
            let source_node_index = &my_graph[edges_].source();
            let target_node_index = &my_graph[edges_].target();
            let source = &my_graph[source_node_index.to_owned()];
            let target= &my_graph[target_node_index.to_owned()];
            let edges_weight = &my_graph[edges_].weight;
            out_file_buffer.write(format!("{}\t{}\t{}\n",
                source.weight,
                target.weight,
                edges_weight).as_bytes());
            //let this_edges = my_graph[edges_ix];

            //edges_.weight.get_weigth();
        }

}