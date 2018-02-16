
/// if node does not exist
/// create it uptade hashmap
/// Create a node add it to the hash_table
/// index
fn get_index(
    my_graph: &mut petgraph::Graph<NodeAttr, EdgesAttr, petgraph::Undirected>,
    my_map: &mut FnvHashMap<String, petgraph::graph::NodeIndex>,
    key: String,
) -> petgraph::prelude::NodeIndex {
    match my_map.get(&key) {
        Some(&number) => number, // the node exist we return the corresponding index
        None => {
            let node_index = my_graph.add_node(NodeAttr {
                name_real: key.to_string(),
            });
            my_map.insert(key, node_index);
            node_index
        }
    }
}


/// return the minimum from two f32 
/// i should use generic..;
///```rust
/// assert_eq!(min_f32(12.0215,0.0001), 0.0001);
///```
fn min_f32(a: f32, b: f32) -> f32 {
    if a > b {
        b
    } else {
        a
    }
}

/// Round to 2 decimal points
///
/// ```rust
/// assert_eq!(my_round(12.456), 12.46);
/// ```
fn my_round(value: f32) -> f32 {
    (value * 100.0).round().trunc() / 100.0
}

/// compute a coverage based on three &str value
/// return a rounded value at 2 decimal point
fn compute_cov(start: &str, end: &str, len: &str) -> f32 {
    let start_f = start.parse::<f32>().unwrap();
    let end_f = end.parse::<f32>().unwrap();
    let length_f = len.parse::<f32>().unwrap();
   // println!("{} {} {}", start_f, end_f, length_f);
    my_round(((end_f - start_f + 1.0) / length_f) * 100.0)
}

///
fn compute_qcov_tcov(vec: &[&str],
header_map: &DicoHeader) -> (f32, f32) {
	
        let qcov = compute_cov(vec[header_map.qstart)], vec[header_map.qend], vec[header_map.qlen]);
       // println!("{}", qcov);
        let tcov = compute_cov(vec[header_map.sstart], vec[header_map.send], vec[header_map.slen]);
        //	println!("{}", tcov);
        (qcov, tcov)
}
