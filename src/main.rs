extern crate renderer;

use renderer::*;

fn main() {
    println!("Basic Renderer.");
    let mut c = Camera::new(
    	Position::new(0.0, 0.0, 0.0), 
    	Quaternion::new(0.707, 0.0, 0.707, 0.0)
    );


    let e = c.rot.euler();
    println!("{:#?}", c);
    println!("{:?}", e);
}
