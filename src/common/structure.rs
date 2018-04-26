extern crate petgraph;

use std::fmt;

// TODO impl display traits


#[derive(Clone)]
pub struct NodeAttr {
    pub name_real: String,
}

impl fmt::Display for NodeAttr{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		write!(f, "{}", self.name_real)
	}
}

impl Default for NodeAttr {
    fn default() -> Self {
        NodeAttr {
            name_real: "default_name_for_testing".to_string(),
        }
    }
}

pub trait is_best<T>{
	fn self_is_best(&self, other: &T) -> bool;
}

pub trait Get_weigth{
	fn get_weigth(&self) -> f32;
	}
	
#[derive(Copy, Clone)]
pub struct LigthEdges{
	pub weight:f32,
}

impl is_best<LigthEdges> for LigthEdges{
	fn self_is_best(&self, other: &LigthEdges) -> bool{
		if self.weight >= other.weight{return true;}
		else {return false;}
	}

}

impl Get_weigth for LigthEdges{
	fn get_weigth(&self) -> f32{return self.weight}
	}

impl fmt::Display for LigthEdges{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		write!(f, "{}", self.weight)
	}
}

#[derive(Copy, Clone)]
pub struct EdgesAttr {
    pub eval: f64,
    pub pid: f32,
    pub cov: f32, // minimum coverage
}



impl is_best<EdgesAttr> for EdgesAttr{
	fn self_is_best(&self, other: &EdgesAttr) -> bool{
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



impl Get_weigth for EdgesAttr{
	fn get_weigth(&self) -> f32{return self.pid}
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

impl fmt::Display for EdgesAttr{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		write!(f, "{:e}\t{}\t{}", self.eval, self.pid, self.cov)
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
		
	pub fn pass_threshold(&self, threshold_struct: &EdgesAttr) -> bool{ // i give it a try I think it will be way faster than assign each times
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


impl fmt::Display for EdgesAttrFull{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		write!(f, "{:e}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
			   self.eval,self.pid, self.qstart, self.qend, self.qlen,
				self.sstart, self.send, self.slen)
	}
}

impl Get_weigth for EdgesAttrFull{
	fn get_weigth(&self) -> f32{return self.pid}
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

// TODO nicer
impl fmt::Display for DicoHeader{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
			   self.qid, self.sid, self.eval, self.bitscore,
		self.pid, self.qstart, self.qend, self.qlen,
		self.sstart, self.send, self.slen)
	}
}
impl DicoHeader{
	
	pub fn from_string(my_string: String) -> DicoHeader{ // a bit long but necessary
		
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
	



