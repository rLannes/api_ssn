#[derive( Clone)]
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


//Structure field are realy more fast to acces than dico ones
#[derive(Copy, Clone)]
pub struct DicoHeader{
	qid: i8,
	sid: i8,
	eval: i8,
	bitscore: i8,
	pid: i8,
	qstart: i8,
	qend: i8,
	qlen: i8,
	sstart: i8,
	send: i8,
	slen: i8,
	}


impl Default for DicoHeader{
	fn default() -> Self{
		DicoHeader{
			qid: -1,
			sid: -1,
			eval: -1,
			bitscore: -1,
			pid: -1,
			qstart: -1,
			qend: -1,
			qlen: -1,
			sstart: -1,
			send: -1,
			slen: -1,
			// mincov tcov qcov
			}
		}
	}


impl DicoHeader{
	
	fn from_string(my_string: String) -> DicoHeader{ // a bit long but necessary
		
		let mut this_dico_header = DicoHeader::default();
		let vec_of_my_string: Vec<_> = my_string.trim().split_whitespace().collect();  // strip() split() my_string
		
		for (i, elem) in vec_of_my_string.iter().enumerate(){ // here i is usize
			
			if elem == &"qseqid"{
				this_dico_header.qid = i as i8; 
				}
				
			else if elem == &"sseqid"{
				this_dico_header.sid = i as i8;
				}
				
			else if elem == &"pident"{
				this_dico_header.pid = i as i8;
				}
			
			else if elem == &"bitscore"{
				this_dico_header.bitscore = i as i8;
				}
			
			else if elem == &"evalue"{
				this_dico_header.eval = i as i8;
				}
			
			else if elem == &"qstart"{
				this_dico_header.qstart = i as i8;
				}
			
			else if elem == &"qend"{
				this_dico_header.qend = i as i8;
				}
			
			else if elem == &"qlen"{
				this_dico_header.qlen = i as i8;
				}
				
			else if elem == &"sstart"{
				this_dico_header.sstart = i as i8;
				}
				
			else if elem == &"send"{
				this_dico_header.send = i as i8;
				}
				
			else if elem == &"slen"{
				this_dico_header.slen = i as i8;
				}
			}
		return this_dico_header
		}
	}
