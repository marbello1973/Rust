use std::any::type_name;

fn main() {
    
    let days = ["Lunes","Martes","Miercoles","Jueves","Viernes","Sabado","Domingo"];
    let lunes = days[0];
    
    if days.contains(&"Mes"){
        println!("Lunes")
    }else{
        println!("No contiene Lunes")
    }


    println!("{}", lunes);    

    for i in lunes.chars() {
        println!("{}", i);
    }

    //busca la letra que aparece en un dia de la semana

    for (index, day) in days.iter().enumerate() {
        let mut positions = Vec::new();
        for (char_index, c) in day.chars().enumerate(){
            if c == 'e'{
                positions.push(char_index);
            }
        }

        if !positions.is_empty(){
            println!(" la letra 'e' , aparece en el dia '{}' en la posicion {:?}.",day, positions);
        }
    }



    let varios_numeros = vec![0,15,25,6];
    println!("inicilizando el vector {:?}", varios_numeros);

    let zeros = vec![0; 10];
    println!("Zeros {:?}", zeros);    

    // creamos un nuevo vector mutable 
    let mut frutas = Vec::new();

    //Gregamos contenido al vector solo se deben agregar valores de tipo str
    frutas.push("Apple");
    frutas.push("Manzana");

    //ejemplo al agregar un entero arroja el siguiente error
    // frutas.push(1);
    // ---- ^ expected `&str`, found integer
    // frutas.push(1);

    //Imprimimos el vector 
    println!("Frutas: {:?}", frutas);

    //Indexacion de un vector 
    let acceder_posicion_3 = varios_numeros[3];
    println!("La posicion numero tres es: {:?}", acceder_posicion_3);

    


}
