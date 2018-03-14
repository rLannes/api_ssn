extern crate fnv;
extern crate petgraph;
extern crate rand;
extern crate easy_cartesians;

use std::time::Instant;

pub mod common;
pub mod read_write;
pub mod algo;

//use common::


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
