
/// vector in polar coordinate(force: f32, angle: f32)
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
		let (a_x, a_y) = self.get_cartesian_coordinate();
		let (b_x, b_y) = self.get_cartesian_coordinate();

		let new_x = a_x + b_x;
		let new_y = a_x + b_y;

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

	pub fn get_x(&self) -> f32{
		let x = self.angle.cos() * self.force;
		x
	}

	pub fn get_y(&self) -> f32{
		let y = self.angle.sin() * self.force;
		y
	}

	pub fn get_cartesian_coordinate(&self) -> (f32, f32){
		return (self.get_x, self.get_y);
		}


	}

#[test]
fn test_vec(){
	let null_vector = MyVector{force:0.0, angle:0.0};
	let a_vec = MyVector{force:1, angle:0.8};


}