use ::std::io;
fn main() {
    // convertir de celsius a fahrenheit

    let mut valor = String::new();

    println!("Ingrese valor a convertir a fahrenheit");

    io::stdin()
        .read_line(&mut valor)
        .expect("Fallo al leer la linea");

    let valor: f32 = match valor.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    if valor == 0.0 {
        println!("El valor ingresado no es valido");
        return;
    }

    let fahrenheit = (valor - 32.0) * (5.0 / 9.0);

    println!("El valor en fahrenheit es: {fahrenheit}°F");
}
