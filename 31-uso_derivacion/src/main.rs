use std::fmt;

//Rust puede implementar automáticamente los rasgos Debug y PartialEq 
//mediante el atributo #[derive(Trait)] si cada uno de sus campos implementa el rasgo:
#[derive(Debug, PartialEq)]
struct Point{
    x: i32,
    y: i32,
}

//No obstante, podemos implementar el rasgo Display para nuestro tipo 
//por nosotros mismos:
//println!("Implemtando el rango Display: ");
//println!("{}", p1);

impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}, {}", self.x, self.y)
        }
}


fn main() {
    let p1 = Point {x: 1, y: 2};
    let p2 = Point {x: 3, y: 5};
    if p1 == p2 {
        println!("Iguales...");
    }else {
        println!("Diferentes...!");
    }

    //para que compile hay que implementar modo display arriba com fmt
    println!("{}", p1);

    //El rasgo Debug, que permite dar formato a un tipo mediante el 
    //especificador de formato {:?}, se usa en
    println!("{:?}", p1);

    
   
}
