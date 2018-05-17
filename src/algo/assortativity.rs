extern crate petgraph;
extern crate fnv;


use std::path;
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use petgraph::Graph;
use std::iter::FromIterator;

use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use std::path::Path;
use std::iter::Iterator;




struct SquaredMat{
    size: u32,
    data: Vec<f32>,
}

impl SquaredMat{


    fn transform_coordinate(&self, position: (u32, u32)) -> usize {
        (self.size*position.0 + position.1) as usize
    }

    fn from_x_get_pos(&self, value: u32) -> (u32, u32){
        let col = (value as f32  % self.size as f32) as u32;
        let row = (value as f32 / self.size as f32).trunc() as u32;
        (row, col)
    }

    fn new(sized: u32) -> SquaredMat {

        let data_size = (sized * sized) as usize;
        let mut my_vec: Vec<f32> = Vec::with_capacity(data_size);
        for i in 0..data_size{
            my_vec.push(0.0);
        }
        let my_mat = SquaredMat {
            size: sized,
            data: my_vec,
        };
        return my_mat;
    }

    ///(row, col)
    fn get_value(&self, position: (u32, u32)) -> f32{
        self.data[self.transform_coordinate(position)]
    }

    fn change_value(&mut self, position: (u32, u32), value: f32) -> (){
        let new_position = self.transform_coordinate(position);
        self.data[new_position] = value;
    }

    fn trace(&self) -> f32 {
        let mut sum = 0.0f32;
        for i in 0..self.size{
            sum += self.get_value((i, i));
        }
        return sum;
    }

    fn add_value(&mut self, position:(u32, u32), value:f32){
        let old = self.get_value(position);
        self.change_value(position, value + old);

    }

    fn squared(&self) -> SquaredMat {
        let size_ = self.size;
        let data_size = (size_* size_) as usize;
        let mut my_vec: Vec<f32> = Vec::with_capacity(data_size);

        for indice in 0..size_ {
            let (row, col) = self.from_x_get_pos(indice);
            let mut sum = 0.0;
            for sub_indices in 0..size_ {
                println!("{} {} {} {}", row, col, sub_indices, indice);
                sum += self.get_value((row, sub_indices)) * self.get_value((sub_indices, col));
            }
            my_vec.push(sum);
        }
        let mut mymat = SquaredMat{
            size: size_,
            data: my_vec,
        };

        return mymat;
    }

    fn get_somme(&self) -> f32{
        self.data.iter().sum()
    }

    fn to_prop(&mut self) ->(){

        let sum: f32 = self.data.iter().sum();
        for i in 0..self.size{
            self.data[i as usize] /= sum;
        }
    }

    fn un_diagonalise_from_less(&mut self) -> (){
        let size_ = self.size;
         for indice in 0..size_ {
            let (row, col) = self.from_x_get_pos(indice);
             if row > col{
                 let value = self.get_value((col, row));
                 self.add_value((row, col), value);
             }
         }

    }

}

//only_those_labels:Option<Vec<String>>,
pub fn graph_assorativity_from_hashmap_label<U: fmt::Display, T: Copy>
                    (
                     my_graph: &Graph<U, T, petgraph::Undirected>,
                     map_annotation: &FnvHashMap<String, String>,
                   map_matrices: &FnvHashMap<String, u32>) -> f32{


    let mut my_mat =  SquaredMat::new(map_matrices.len() as u32);

    println!("size {}: ", map_matrices.len() as u32);

    for edges in my_graph.raw_edges()
        {
            let source = edges.source();
            let target = edges.target();
            let annot_source = map_annotation.get(&my_graph[source].to_string());
            let annot_target = map_annotation.get(&my_graph[target].to_string());
            if !map_matrices.contains_key(&annot_source.unwrap().to_owned()) || !map_matrices.contains_key(&annot_target.unwrap().to_owned()){
                continue;
            }

            let col = map_matrices.get(&annot_source.unwrap().to_owned()).unwrap();
            let row = map_matrices.get(&annot_target.unwrap().to_owned()).unwrap();
            if row <= col {
                my_mat.add_value((*row, *col), 1.0f32);
            }
            else{
                my_mat.add_value((*col, *row), 1.0f32);
            }
        }

    my_mat.to_prop();
    let squared_sum = my_mat.squared().get_somme();
    (my_mat.trace() - squared_sum) / (1.0 - squared_sum)

}


