/// layout
/// a force directed layout that use a gravity like to have round shapedform
///
extern crate small_cartesians_lib;
extern crate fnv;
extern crate petgraph;
extern crate rand;

use fnv::FnvHashMap;
use small_cartesians_lib::module::d2;

use layout::common_layout;
use read_write::write_svg::write_graph_svg;
use petgraph::visit::EdgeRef;

use fnv::FnvHashSet;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use common::function::{max_f32, min_f32};
use common::structure::{Get_weigth};

use std::cell::RefCell;
use std::f32;
use std::path::Path;

use rand::distributions::{IndependentSample, Range};
use algo::ConnectedComponent;

//TODO make the multi Connected Component and write a organise square module
//
//SPRING_STIFF = 10        #Raideur ressort
//SPRING_REST = 40        #Longueur a vide pour une arete
//SPRING_REST2 = 400      #longueur a vide pour une absence d'arete
//EXPONENT = 1.4          #Non linear recoil
//DRAG = 10                #force de frottement
//MIN_RADIUS = 50         #rayon de repulsion des noeuds
//REPULSE = 5             #force de repuslion
//GRAV = 50                #Gravité
//SHAKE = 25              #•deplacmeent recuit
//DELTA_T = 0.01          #Pas de temps de la simulation
//MAXSTEP = 80            #Deplacement MAX
struct force_directed_parameter{
    repulse: f32,
    gravity: f32,
    attraction: f32,
    stiff: f32,
    drag: f32,
    radius_repulsion: f32,
    shake: f32,
    delta_t: f32,
    max_step: f32,
    exponent: f32,
}

/// compute it for one Connected Component
pub fn force_directed_layout<'a, T: Get_weigth  , U>(my_graph: &petgraph::Graph<U, T, petgraph::Undirected>,
                                                     f_param: &force_directed_parameter,
                                                     iter_num: f32, end_threshold:f32, node_cc: Vec<petgraph::prelude::NodeIndex>)
                                -> (D2::Rectangle, FnvHashMap<petgraph::graph::NodeIndex, D2::Vect2D>){
    // will give the boundary of the  rectangle that include all nodes used by plot function
    let this_rectangle = Rectangle{up_left: d2::Vect2D{x:0.0, y:0.0},
								down_rigth: d2::Vect2D{x:0.0, y:0.0}};

    let number_nodes = node_cc.len();
     let array_node1 = common_layout::init_position(node_cc,
                                                    100f32,
                                                    100f32);
    let array_node2 : [&mut d2::Vect2D; number_nodes];

}