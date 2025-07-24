fn main() {

    let mut contador = 1;

    let stop_loop = loop {
        contador *= 2;
        if contador > 100 {
            break contador;
        } 
    };

    println!("Contador: {}", stop_loop);

    let mut contador_dos = 1;
    // bucle while 
    while contador_dos < 5 {
        contador_dos = contador_dos + 1;
        println!("Contador dos while {}: ", contador_dos);
    }

    // ciclo for 
    let frutas = ["manzana", "pera", "uva", "lulo"];
    for fruta in frutas  {
        println!("Frutas: {}", fruta);
    }

    for number in 0..5 {
        println!("Numeros: {}", number * 2);
    }


} 
