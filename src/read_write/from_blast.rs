use std::path::Path;

/// read a blast file
/// TODO filter value option
///return hashmap, graph <edgesAtrr (min(cov), pid, eval), Nodeattr(name_real)>
pub fn read_from_blast(header: &structure::DicoHeader, file: &Path, threshold_values:Options<EdgeAtrr>){

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
		// before going further we check if edges go
		match threshold_values{
			some(threshold) => if !this_edges_properties.pass_threshold(threshold){continue}
			}
		
		
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
