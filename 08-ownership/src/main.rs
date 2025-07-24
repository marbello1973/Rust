fn main() {
    // asignacion de memoria en el heap en un cadena string
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    // tomar ownership
    tomar_ownership(s);

    let x = 5;
    //let y = x;

    hacer_copia(x);

    
    let s1 = String::from("Hola");
    let s2 = s1.clone();
    println!("S1: {s1}, S2: {s2}"); // error    
    
    let mut s3 = String::from("Hola");
    let len = calcular_longitud(&s3);
    println!("La longitud de {s1} es {len}");

    modificar(&mut s3);
    
    // ejemplo de dos refrencias mutables envia error de compilacion
    {
        let r1 = &mut s3;            
        println!("Otro scope mutable {} ", r1);
    }

    {
        let r2 = &mut s3;
        println!("Otro valor mutable entre llaves {}", r2);
            
    }


    let r3 = &s3;
    let r4 = &s3;
    println!("Variable con referencia {}, {}", r3, r4);
    
    referencia_mutable();
    
    // llamado a la funcion con referencia colgante 
    //let ref = colgante();
    //println!("colgante: {ref}");


    let no_colgante_ref = no_colgante();
    println!("no colgante referencia {no_colgante_ref}");

    let f1 = String::from("hola mundo");
    let first = first_word(&f1);
    println!("First word: {}", first);

    
    let my_string = String::from("Hola Mundo");
    let _word = first_word(&my_string);


}


fn calcular_longitud(s: &String) -> usize {
    s.len()
}

// modificar &mut s para quitar el error 
// un_string.push_str(", mundo!");
//     ^^^^^^^^^ `un_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable 
fn modificar(un_string: &mut String){
    un_string.push_str(", mundo!");
}

fn tomar_ownership(un_string: String) {
    println!("{un_string}");
} // s sale del scope y se libera la memoria

fn hacer_copia(un_entero: i32){
    println!("{un_entero}");
} // s sale del scope pero se devuelve y se copia la memoria

fn referencia_mutable(){
    let mut s = String::from("referencia mutable");

    let r1 = &s;
    let r2 = &s;
    println!("Referencia mutable: {}, {}", r1, r2);

    let r3 = &mut s;
    println!("Otra refrencia mutable s: {r3}")
}

//  referencia colgante 
/*fn colgante() -> &String {
    let s = String::from("ref colgante");
    &s
}*/

fn no_colgante() -> String {
    let s = String::from("No colgante ref");
    s
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}





