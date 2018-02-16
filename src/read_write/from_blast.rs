use std::path::Path;

/// read a blast file
/// TODO filter value option
///return hashmap, graph <edgesAtrr (min(cov), pid, eval), Nodeattr(name_real)>
pub fn read_from_blast(header: &structure::DicoHeader, file: &Path, threshold_values:Options<EdgeAtrr>){

	let my_graph = Graph::<NodeAttr, EdgesAttr, petgraph::Undirected>::new_undirected();
        
	let my_name_to_index_hashmap: FnvHashMap<String, petgraph::graph::NodeIndex> =
        FnvHashMap::with_capacity_and_hasher(1_000_000, Default::default());
        
   let in_file = File::open(path_in_file).unwrap_or_else(|why| {
	panic!(
		"couldn't open {}: {}",
		path_in_file.display(),
		why.description()
	)
    });
    
    //ToDO clean this sepearte in other foo()
    let mut  in_file_buffer = BufReader::with_capacity(60_000, in_file); //bufering
    
    for lines in in_file_buffer{
		
		let v_line: Vec<_> = my_header_str.trim() // split line into vector
								.split_whitespace()
								.collect();
		
		let this_edges_properties = get_std_edges_attributs(&v_line, header);
		// before going further we check if edges pass threshold
		match threshold_values{
			some(threshold) => if !this_edges_properties.pass_threshold(threshold){continue},
			}
		
		// get index return the graph index of a nodes , and create a node and add it to the graph if it don't exist 
		let index1 = function.get_index(my_graph, my_name_to_index_hashmap, v_line[header.qid].to_string())
		let index2 = function.get_index(my_graph, my_name_to_index_hashmap, v_line[header.qid].to_string())
		// then if edges don't exist or is of lower quality update it
		
		}
	}

/*
pub fn read_from_blast_only_edges(header: &structure::DicoHeader, file: &Path){

   let in_file = File::open(path_in_file).unwrap_or_else(|why| {
	panic!(
		"couldn't open {}: {}",
		path_in_file.display(),
		why.description()
	)
    });
    
    //ToDO clean this sepearte in other foo()
    let mut  in_file_buffer = BufReader::with_capacity(60_000, in_file); //bufering
    
    for lines in in_file_buffer{
		
		let v_line: Vec<_> = my_header_str.trim() // split line into vector
								.split_whitespace()
								.collect();
		
		
		}
	}


pub fn read_from_blast_big_edges_info(){
	
	}
*/
