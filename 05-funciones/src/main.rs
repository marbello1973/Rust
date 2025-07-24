

fn main() {
    println!("Hello, world!");
    let y:i32=51; 
    another_function(y);

    multiples_parametros(5, 'h');

    let f = five();
    println!("valor de la funcion con retorno {f}");

    let x = plus_one(5);
    println!("otro ejemplo {x}");
}

fn another_function(x: i32){
    println!("Another function {x}");
}

fn multiples_parametros(value: i32, unit_label: char){
    println!("varios parametros en la funcion {value} - {unit_label}")
}

// funciones que devuelve un valor o retorna un valor 
fn five() -> i32{
    // se pude solo declarar el numero 5 tiene un return implicito
    // 5
    // o se pude usar return 
    return 5;
}

fn plus_one(x: i32) -> i32{
    x + 1 //sin punto y coma tiene un return implicito
}