use std::io;

fn main() {
    let mut filas_str = String::new();
    let mut columnas_str = String::new();
    let mut datos: Vec<Vec<i32>>;

    println!("Ingrese el numero de filas:");
    io::stdin().read_line(&mut filas_str).expect("error");

    println!("Ingrese el numero de columnas:");
    io::stdin().read_line(&mut columnas_str).expect("error");

    let filas = filas_str.trim().parse::<i32>().expect("error");
    let columnas = columnas_str.trim().parse::<i32>().expect("error");

    datos = vec![vec![0; columnas as usize]; filas as usize];

    // Llenar la matriz con valores ingresados por el usuario
    for i in 0..filas {
        for j in 0..columnas {
            let mut valor_str = String::new();
            println!("Ingrese el valor para la posición [{}, {}]:", i, j);
            io::stdin()
                .read_line(&mut valor_str)
                .expect("Error al leer el valor");
            datos[i as usize][j as usize] = valor_str
                .trim()
                .parse::<i32>()
                .expect("El valor debe ser un número entero");
        }
    }

    println!("Matriz: {:?}x{:?}", filas, columnas);

    for i in 0..filas as usize {
        println!("{:?}", datos[i as usize]);
    }
}
