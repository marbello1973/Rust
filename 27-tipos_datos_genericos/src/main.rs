// tipos de datos genericos definimos una structura que contenga cualquier tipo de dato<T>
// de la misma variante, ejemplo todos boleanos, enteros o float, no se permite cobinar datos

#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}

// esta estruct si permite combinar tipoas de datos 
#[derive(Debug)]
struct Point1<T, U>{
    w: T,
    z: U,
}

// los warnig es porque no estan siendo usados 

fn main() {
    let work = Point {x: true, y: false};
    println!("X: {}, Y: {}", work.x, work.y);

    let work1 = Point1 {w: 5, z: "hola"};
    println!("W: {}, Z: {}", work1.w, work1.z);

}
