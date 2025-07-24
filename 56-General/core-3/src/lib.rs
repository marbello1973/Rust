use std::io;

pub fn figuras_geometricas() {
    let variable1: i32 = 10;
    let ancho: i32 = 10;
    let alto: i32 = 5;
    let lado: i32 = 10;
    let mut simbolo: String = String::new();
    let mut simbolo2: String = String::new();
    let mut simbolo3: String = String::new();

    io::stdin().read_line(&mut simbolo).expect("Error");
    let simbolo: &str = simbolo.trim();

    for i in 1..variable1 {
        let esp = " ".repeat((variable1 - i) as usize);
        let ascci = simbolo.repeat((2 * i - 1) as usize);
        println!("{}{}", esp, ascci);
    }

    io::stdin().read_line(&mut simbolo2).expect("Error");
    let simbolo2: &str = simbolo2.trim();

    for i in 0..alto {
        if i == 0 || i == alto - 1 {
            println!("{}", simbolo2.repeat(ancho as usize));
        } else {
            println!("{simbolo2}{}{simbolo2}", " ".repeat((ancho - 2) as usize))
        }
    }

    io::stdin().read_line(&mut simbolo3).expect("Error");
    let simbolo3: &str = simbolo3.trim();

    for _ in 0..lado {
        println!("{}", simbolo3.repeat(lado as usize));
    }

    //Ejemplo de clone
    let sal1: String = String::from("value");
    let sal2: String = sal1.clone();
    println!("sal1: {sal1}, sal2: {sal2}");
}
