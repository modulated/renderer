use std::f64::consts::PI;

#[derive(Debug)]
pub struct Position {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl Position {
	pub fn new(x: f64, y: f64, z: f64) -> Position {
		Position {x, y, z}
	}
}

#[derive(Debug)]
pub struct Quaternion {
	pub w: f64,
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl Quaternion {
	pub fn new(w: f64, x: f64, y: f64, z: f64) -> Quaternion {
		Quaternion {w, x, y, z}
	}

	pub fn euler(&self) -> (f64, f64, f64) {
		let test = self.x * self.y + self.z * self.w;

		if test > 0.49999 {
			println!("pos lock");
			let heading:f64 = 2.0 * self.w.atan2(self.x);
			let pitch:f64 = PI/2.0;
			let bank:f64 = 0.0;
			return (heading.to_degrees(), pitch.to_degrees(), bank.to_degrees());
		}

		if test < -0.49999 {
			println!("neg lock");
			let heading:f64 = -2.0 * self.w.atan2(self.x);
			let pitch:f64 = PI/2.0;
			let bank:f64 = 0.0;
			return (heading.to_degrees(), pitch.to_degrees(), bank.to_degrees());
		}

		let sqx:f64 = self.x * self.x;
		let sqy:f64 = self.y * self.y;
		let sqz:f64 = self.z * self.z;
		
		let heading:f64 = (2.0 * self.y * self.w - 2.0 * self.x * self.z).atan2(1.0 - 2.0 * sqy - 2.0 * sqz) ;
		let pitch:f64 = (2.0 * test).asin();
		let bank:f64 = (2.0 * self.x * self.w - 2.0 * self.y * self.z).atan2(1.0 - 2.0 * sqx - 2.0 * sqz);

		(heading.to_degrees(), pitch.to_degrees(), bank.to_degrees())
	}
}

#[derive(Debug)]
pub struct Camera {
	pub pos: Position,
	pub rot: Quaternion
}

impl Camera {
	pub fn new(pos: Position, rot: Quaternion) -> Camera {
		Camera {pos, rot}
	}
}