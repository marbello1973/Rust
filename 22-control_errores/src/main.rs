struct Persona {
    first: String,
    middle: Option<String>,
    last: String
}

fn build_full_name(persona: &Persona) -> String{
    let mut full_name = String::new();
    full_name.push_str(&persona.first);
    full_name.push_str(" ");

    if let Some(middle) = &persona.middle {
    full_name.push_str(&middle);
    full_name.push_str(" ");
    }

    full_name.push_str(&persona.last);
    full_name
}



fn main() {
    //panic!("Firewall error de variables...!");
    //let v = vec![0,1,2,3];
    //println!("{}", v[6]);
    
    let frutas = ["banana", "manzana", "pera", "uva", "melon"];

    let first = frutas.get(0);
    println!("{:?}", first);

    let non_exists = frutas.get(99);
    println!("{:?}", non_exists);

    for &index in [0, 1, 2, 3, 99, 88, 57, 4].iter(){
        match frutas.get(index) {
            Some(&"manzana") => println!(" Manzanas son deliciosas!!!"),
            Some(frutas_name) => println!("Frutas nombre: {}..!", frutas_name),
            None => println!("No hay frutas en este index :( {:?}", &index),
        }
    }

    // patron de caracter comodin _ subrayado para ignorar la variante None
    let a_number: Option<u8> = Some(7);
    match a_number {
        Some(7) => println!("numero existe"),
        _ => {},
    }

    // if let compara patron con una expresion
    let b_number: Option<u8> = Some(8);
    if let Some(8) = b_number {
        println!("Numero 8 existe ..!");
    }

    // Uso de unwrap y expect
   /* let gif = Some("candy");
    assert_eq!(gif.unwrap(), "candy");
    let empty_gif:  Option<&str> = None;
    assert_eq!(empty_gif.unwrap(), "candy");
*/
/*    let a = Some("value");
    assert_eq!(a.expect("frutas no existe"), "value");

    let b: Option<&str> = None;
    b.expect("None");
*/

//Como estas funciones pueden emitir alertas de pánico, no se recomienda usarlas. 
//Considere mejor uno de los siguientes enfoques:
//Use la coincidencia de patrones y administre el caso None explícitamente.
//Llame a métodos similares que no emiten alertas de pánico, 
//como unwrap_or, que devuelve un valor predeterminado 
//si la variante es None o el valor interno si la variante es Some(value).

    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");

}
