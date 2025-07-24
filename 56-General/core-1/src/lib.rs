use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn adivina_numero() {
    let numero_secreto = rand::rng().random_range(1..101);

    let mut numero_intentos: i32 = 0;
    let maximo_intentos: i32 = 10;

    loop {
        println!("Ingresa tu número...!");

        let mut num: String = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Error al leer la entrada");

        println!("Numero ingresado: {}", num.trim());

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Introduzca un valor valido");
                continue;
            }
        };

        numero_intentos += 1;

        let porcentaje: f64 = (numero_intentos as f64 / maximo_intentos as f64) * 100.0;
        //println!("Porcentaje acumulado: {porcentaje}");
        if numero_intentos >= maximo_intentos {
            println!("Supero maximo de intentos: {maximo_intentos} - {porcentaje}%");
            break;
        }

        match num.cmp(&numero_secreto) {
            Ordering::Less => println!("¡Numero ingresado muy pequeño...!"),
            Ordering::Greater => println!("¡Numero ingresado muy grande...!"),
            Ordering::Equal => {
                println!("¡Has Adiviniado...{numero_secreto}.!");
                println!(
                    "Lo has logrado en {numero_intentos}: intentos - {porcentaje}%: participacion"
                );
                break;
            }
        }
    }
}
