extern crate fnv;
extern crate petgraph;
extern crate rand;
extern crate small_cartesians_lib;

use std::time::Instant;

pub mod common;
pub mod read_write;
pub mod algo;
pub mod layout;

//use common::


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
