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

pub trait Get_weigth{
	fn get_weigth(&self) -> f32;
	}
	
#[derive(Copy, Clone)]
pub struct EdgesAttr {
    pub eval: f64,
    pub pid: f32,
    pub cov: f32, // minimum coverage
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


pub struct position{
	pub x:f32,
	pub y:f32,
	}
	
impl position{
	
	pub fn get_position(&self) -> (f32, f32){
		return (self.x, self.y)
		}
		
	pub fn distance(&self, b_point: &position) -> f32{
		return ((b_point.x - self.x).powf(2.0) +  (b_point.y - self.y).powf(2.0)).sqrt();
		}
		
	pub fn bearing_angle(&self, b_point: &position) -> f32{
		let tan_teta = (b_point.x - self.x) / (b_point.y - self.y);
		tan_teta.atan()
		}
	
	
	pub fn add_vector(&mut self, vector: &MyVector){
		let _composante = vector.get_composantes();
		self.x += _composante.0;
		self.y += _composante.1;
		}
	
	}

	
	
/// vector(force: f32, angle: f32)
/// angle are in radiant
#[derive(Copy, Clone)]
pub struct MyVector{
	pub force: f32,
	pub angle: f32,// in radians
	}
	

impl MyVector{
	
	
	pub fn enclosing_angle(&self, vec_: &MyVector) -> f32 {
		let dot_product = self.force * self.angle.sin()  + vec_.force * vec_.angle.sin();
		let product_magnitude = self.force * vec_.force;
		dot_product / product_magnitude
		}
		
	pub fn somme(&self, vec_: &MyVector) -> MyVector{
		let cos_angle_ = self.enclosing_angle(vec_);
		let new_force = self.force.powf(2.0)  + vec_.force.powf(2.0) - 2.0 * self.force * vec_.force * cos_angle_;
		let angle = cos_angle_.acos();
		MyVector{ force: new_force,
				   angle: angle }
		}
		
	pub fn scalar(&self, vec_: &MyVector) -> f32{
		let cos_angle_ = self.enclosing_angle(vec_);
		self.force * vec_.force * cos_angle_	
		}
		
	pub fn get_composantes(&self) -> (f32, f32){
		let x = self.angle.cos() * self.force;
		let y = self.angle.sin() * self.force;
		return (x, y);
		}
	}
	

pub struct rectangle{
	pub position_up_left: position,
	pub position_down_rigth: position
	}



