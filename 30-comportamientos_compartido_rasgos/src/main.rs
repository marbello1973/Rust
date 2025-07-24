mod geometria; //declaramos el modulo geometria;

// importamos los archivos
use geometria::figuras::{Circle, Rectangle};
use geometria::rasgos::Area;

fn main() {
    let num1: f64 = 0.25;
    let num2: f64 = 0.0145;
    let num3: f64 = 25.23;
    let circle = Circle { radius: num1};
    let rectangle = Rectangle { width: num2, height: num3 };
    println!("Area del circulo: {}", circle.area());
    println!("Rectangulo del Area: {}", rectangle.area());
}
