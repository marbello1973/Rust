// #[macro_use]
// extern crate nalgebra as na;
// 
// use na::{Vector3, Rotation3};

use nalgebra::{Vector3, Rotation3, Vector4};

use termion::color;

#[derive(Debug)]
struct Cubo { 
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Cubo {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z, w:0 }
    }

    // unir las coordenadas con rayas x,y,z,w
   
    

}

fn main() {

    let axis  = Vector3::x_axis();
    let angle = 1.57;
    let b     = Rotation3::from_axis_angle(&axis, angle);

 
    println!("b: {:?}", b   );

    println!("{}", color::Fg(color::Red));
    

    let p1 = Cubo::new(1, 2, 3);
    println!("Cubo: {}:{}:{}:{}", p1.x, p1.y, p1.z, p1.w);


    let vect = Vector4::new(1.0, 2.0, 3.0, 4.0);

    println!("Vector: {:?}", vect);

    
}