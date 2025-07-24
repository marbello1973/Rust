mod longitud;

fn main() {
    let s1 = String::from("Hola, Mundo...!");
    let len = longitud::longitud(&s1);
    println!("Longitud de: {s1} es {len}.");
}

