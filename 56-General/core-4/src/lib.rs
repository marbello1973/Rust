// Challneger de canguros si se encuentran en salto
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    println!("canguro");
    // Ejemplo de lógica para determinar si los canguros se encontrarán
    if v1 != v2 && (x2 - x1) % (v1 - v2) == 0 && (x2 - x1) / (v1 - v2) >= 0 {
        return "YES".to_string();
    } else {
        return "NO".to_string();
    }
}

// 'static se refiere al tiempo de vida de la referencia que se esta devolviendo y si es valida
// &'static str cadena de caracter a la que hago referencia...
pub fn get_percentage_rounds(percentage: f64) -> &'static str {
    // Definimos un array de cadenas para cada rango de porcentaje
    let rounds = [
        " ",  // 0.0
        "▏",  // 0.0 - 0.1
        "▎",  // 0.1 - 0.2
        "▍",  // 0.2 - 0.3
        "▌",  // 0.3 - 0.4
        "▋",  // 0.4 - 0.5
        "▊",  // 0.5 - 0.6
        "▉",  // 0.6 - 0.7
        "█",  // 0.7 - 0.8
        "█▏", // 0.8 - 0.9
        "█▎", // 0.9 - 1.0
    ];

    // Calculamos el índice basado en el porcentaje
    // as usize --> convierte ese número entero en un usize, que es el tipo adecuado
    // para usarlo como índice en el array rounds
    let index = (percentage * 10.0).floor() as usize;

    // Devolvemos la cadena correspondiente
    rounds.get(index).unwrap_or(&"█▎")
}

pub fn get_totall_x(a: &[i32], b: &[i32]) -> i32 {
    let minimo_valor = *a.iter().max().unwrap();
    let maximo_valor = *b.iter().min().unwrap();
    let mut contador = 0;

    for i in minimo_valor..=maximo_valor {
        let divisible_a = a.iter().all(|&valor_a| i % valor_a == 0);
        let factor_b = b.iter().all(|&valor_b| valor_b % i == 0);
        if divisible_a && factor_b {
            contador += 1;
        }
    }
    contador
}
