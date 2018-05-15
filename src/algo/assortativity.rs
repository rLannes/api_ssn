extern crate fnv;
extern crate petgraph;

use petgraph::Graph;
use std::fmt;
use common::function::get_degree;
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use std::iter::FromIterator;

/// return 0 if node have different annotation or one node label not in annot_set if filter is set
/// else return 1
fn delta_kronecker(node1_annotation: &String, node2_annotation: &String,
                  map_annot: &FnvHashMap<String, String>,
                  filter_label: &bool, annot_set: &FnvHashSet<String>) -> u8 {

    let node1_annot = map_annot.get(node1_annotation);
    let node2_annot = map_annot.get(node2_annotation);
    if node2_annot.is_none() || node1_annot.is_none(){
        panic!("unable to find annotation for at least one of those {} {}", node2_annotation, node1_annotation);
    }
    //println!("kronecker: annot1: {} {}, annot2: {} {}", node1_annotation, node1_annot.unwrap(), node2_annotation, node2_annot.unwrap());
    if *filter_label {
        if !annot_set.contains(node1_annot.unwrap()) || !annot_set.contains(node2_annot.unwrap()) {
            //println!("in filter labels");
            return 0
        }
    }

    if node1_annot != node2_annot{ return 0 }
    else {return 1 }
}


///only_those_labels if Some will only consider node that have specify label
/// map_annotation an hashMap node.to_string() -> annotation
/// my_graph a petgraph Graph
pub fn graph_assorativity_from_hashmap_label<U: fmt::Display, T: Copy>
                    (only_those_labels:Option<Vec<String>>,
                     my_graph: &Graph<U, T, petgraph::Undirected>,
                     map_annotation: &FnvHashMap<String, String>) -> f32{


    // number of vertices
    let mut total_edges = 0.0f32;

    let mut filter_label = true;
    let mut set_annotation = FnvHashSet::with_capacity_and_hasher(100, Default::default());

    match only_those_labels{
        Some(my_vec) => {set_annotation = FnvHashSet::from_iter(my_vec);}
        None => filter_label = false
    }


    if filter_label{
        // get only node matching labels
        for edges in my_graph.raw_edges(){
            let source = edges.source();
            let target = edges.target();

            if set_annotation.contains(&my_graph[source].to_string()) && set_annotation.contains(&my_graph[target].to_string()) {
                total_edges += 1.0;
            }
        }
    }

    else{
       total_edges =  my_graph.edge_count() as f32;
    }


    // there are two main computation du do:
    //
    //sum(Ki*Kj/m)
    let mut somme2 = 0.0f32;
    //sum(Aij -(Ki*Kj/m))
    let mut somme1 = 0.0f32;

    let mut Aij = 0.0f32;

    for node_i in my_graph.node_indices(){

        for node_j in my_graph.node_indices(){

            if node_i == node_j {continue}

            else {
                if my_graph.contains_edge(node_i, node_j){
                    Aij = 1.0;
                }
                else { Aij = 0.0; }

                if delta_kronecker(&my_graph[node_i].to_string(), &my_graph[node_j].to_string(),
        &map_annotation, &filter_label, &set_annotation) == 0 { continue}
                 else {
                    //println!("in_kronecker");
                    let degree1 = get_degree(my_graph, &node_j) as f32;
                    let degree2 = get_degree(my_graph, &node_i) as f32;
                    let degree_product = degree1 * degree2;
                    let intermediare = degree_product / (2.0 * total_edges);
                    somme2 += intermediare;
                    somme1 += (Aij - intermediare);
                    //println!("degree1: {}, degree2: {}, degree_product: {}, intermediare: {}, somm1: {} somm2: {}",
                    //degree1, degree2, degree_product, intermediare, somme1, somme2);
                    }
            }
        }
    }
    //println!("somme1: {}, somme2: {}, total_node: {}",somme1, somme2, total_edges);
    somme1 / ((2.0 * total_edges) - somme2)
}

//TODO make one with node weight directly
