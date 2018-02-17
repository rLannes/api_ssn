extern crate petgraph;

#[derive(Clone)]
pub struct NodeAttr {
    pub name_real: String,
}

impl Default for NodeAttr {
    fn default() -> Self {
        NodeAttr {
            name_real: "default_name_for_testing".to_string(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct EdgesAttr {
    pub eval: f64,
    pub pid: f32,
    pub cov: f32, // minimum coverage
}

impl Default for EdgesAttr {
    fn default() -> Self {
        EdgesAttr {
            eval: 0.0,
            pid: 100.0,
            cov: 100.0,
        }
    }
}

impl EdgesAttr{
	pub fn self_is_best(&self, other: &EdgesAttr) -> bool{
		
		if self.eval < other.eval {return true}
		else if self.eval > other.eval {return false}
		else{if self.pid > other.pid {return true}
			else if self.pid < other.pid {return false}
			else{
				if self.cov > other.cov {return true}
				else if self.cov < other.cov {return false}
				}
			}
		return true
		}
		
	pub fn pass_threshold(&self, threshold_struct: &EdgesAttr) -> bool{ // i giove it a try I think it will be way faster than assign each times
		if self.cov < threshold_struct.cov {return false}
		else if self.pid < threshold_struct.pid {return false}
		else if self.eval > threshold_struct.eval {return false} // for eval the lower the better
		else {return true}
		}
	}


pub struct EdgesAttrFull {
    pub eval: f64,
    pub pid: f32,
    pub bitscore: f32,
    pub cov: f32, // minimum coverage
    pub qstart: i32,
	pub qend: i32,
	pub qlen: i32,
	pub sstart: i32,
	pub send: i32,
	pub slen: i32,
	pub qindex: petgraph::graph::NodeIndex,
	pub sindex: petgraph::graph::NodeIndex,    
}
/*
trait Compare_edges<T>{
		//type prop;
		fn self_is_best(&self, other: &T) -> bool;
	}
*/

impl EdgesAttrFull{
	
	pub fn self_is_best(&self, other: &EdgesAttrFull) -> bool {
		
		if self.eval < other.eval {return true}
		else if self.eval > other.eval {return false}
		else{if self.pid > other.pid {return true}
			else if self.pid < other.pid {return false}
			else{
				if self.cov > other.cov {return true}
				else if self.cov < other.cov {return false}
				}
			}
		return true
		}
		
	}


/*
impl Compare_edges<T> for EdgesAttrFull{
	
	//type prop = EdgesAttrFull;
	
	fn self_is_best<T>(&self, other: &T) -> bool {
		
		if self.eval < other.eval {return true}
		else if self.eval > other.eval {return false}
		else{if self.pid > other.pid {return true}
			else if self.pid < other.pid {return false}
			else{
				if self.cov > other.cov {return true}
				else if self.cov < other.cov {return false}
				}
			}
		return true
		}
		
	}

*/
/**
impl Default for EdgesAttrFull {
    fn default() -> Self {
        EdgesAttr {
            eval: 0.0,
            pid: 100.0,
            cov: 100.0,
            qstart: i32,
	pub qend: i32,
	pub qlen: i32,
	pub sstart: i32,
	pub send: i32,
	pub slen: i32,
	pub qindex: petgraph::graph::NodeIndex,
	pub sindex: petgraph::graph::NodeIndex,    
        }
    }
}
*/
//Structure field are realy more fast to acces than dico ones
#[derive(Copy, Clone)]
pub struct DicoHeader{
	pub qid: usize,
	pub sid: usize,
	pub eval: usize,
	pub bitscore: usize,
	pub pid: usize,
	pub qstart: usize,
	pub qend: usize,
	pub qlen: usize,
	pub sstart: usize,
	pub send: usize,
	pub slen: usize,
	}


impl Default for DicoHeader{
	fn default() -> Self{
		DicoHeader{
			qid: 100,
			sid: 100,
			eval: 100,
			bitscore: 100,
			pid: 100,
			qstart: 100,
			qend: 100,
			qlen: 100,
			sstart: 100,
			send: 100,
			slen: 100,
			}
		}
	}


impl DicoHeader{
	
	fn from_string(my_string: String) -> DicoHeader{ // a bit long but necessary
		
		let mut this_dico_header = DicoHeader::default();
		let vec_of_my_string: Vec<_> = my_string.trim().split_whitespace().collect();  // strip() split() my_string
		
		for (i, elem) in vec_of_my_string.iter().enumerate(){ // here i is usize
			
			if elem == &"qseqid"{
				this_dico_header.qid = i as usize; 
				}
				
			else if elem == &"sseqid"{
				this_dico_header.sid = i as usize;
				}
				
			else if elem == &"pident"{
				this_dico_header.pid = i as usize;
				}
			
			else if elem == &"bitscore"{
				this_dico_header.bitscore = i as usize;
				}
			
			else if elem == &"evalue"{
				this_dico_header.eval = i as usize;
				}
			
			else if elem == &"qstart"{
				this_dico_header.qstart = i as usize;
				}
			
			else if elem == &"qend"{
				this_dico_header.qend = i as usize;
				}
			
			else if elem == &"qlen"{
				this_dico_header.qlen = i as usize;
				}
				
			else if elem == &"sstart"{
				this_dico_header.sstart = i as usize;
				}
				
			else if elem == &"send"{
				this_dico_header.send = i as usize;
				}
				
			else if elem == &"slen"{
				this_dico_header.slen = i as usize;
				}
			}
		return this_dico_header
		}
	}
