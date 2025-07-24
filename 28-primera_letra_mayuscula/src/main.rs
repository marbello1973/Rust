
fn primera_letra_mayuscula(input: String) -> String{
    input
        .split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &c.as_str().to_lowercase()
                }
                None => String::new(),
            }
        }).collect::<Vec<String>>().join(" ")    
}


fn main() {
    let variable = "Hola mundo desde colombia".to_string();
    let t = primera_letra_mayuscula(variable);
    println!("variable {:?}", t);


    let m = "otra cosa diferente de comprar".split_whitespace().map(|i| {
        let mut c = i.chars();
        match c.next() {
            Some(first) => {first.to_uppercase().collect::<String>()}
            None => String::new(),
        }        
    });
    println!("LETRA ASIGNADA M: -> {:?}", m);
}
