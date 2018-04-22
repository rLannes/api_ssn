extern crate rand;
extern crate petgraph;
extern crate fnv;
extern crate small_cartesians_lib;


use fnv::FnvHashMap;
use small_cartesians_lib::module;
use rand::distributions::{IndependentSample, Range};


/// create and return an array with
pub fn init_position(vec_nodes:Vec<petgraph::graph::NodeIndex>, width: f32, height: f32)
    -> [D2::Vect2D; vec_nodes.len()]{

     let between = Range::new(1.0, 10.0);
	 let mut rng = rand::thread_rng();

        let array_: [d2::Vect2D; vec_nodes.len()];
        for i in O..vec_nodes.len(){
            let new_x: f32 = between.ind_sample(&mut rng);
		    let new_y: f32 = between.ind_sample(&mut rng);
            array_[i] = D2::Vect2D{x: new_x, y: new_y};

    }
    return array_;
}