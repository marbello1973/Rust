use std::io;

// La estructura ahora almacena las dimensiones como números y los datos de la matriz.
struct Matriz {
    filas: usize,
    columnas: usize,
    datos: Vec<Vec<i32>>, // Usaremos i32 para los valores de la matriz
}

impl Matriz {
    /// Crea una nueva Matriz a partir de las dimensiones dadas por el usuario.
    fn desde_entrada_usuario() -> Self {
        let filas = Self::leer_dimension("Introduce el número de filas (n):");
        let columnas = Self::leer_dimension("Introduce el número de columnas (m):");

        // Crea una matriz de `filas` x `columnas` inicializada con ceros.
        let datos = vec![vec![0; columnas]; filas];

        Matriz {
            filas,
            columnas,
            datos,
        }
    }

    /// Función auxiliar para leer y parsear una dimensión desde la consola.
    /// Repite la pregunta hasta que se introduce un número válido.
    fn leer_dimension(mensaje: &str) -> usize {
        loop {
            println!("{}", mensaje);
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error al leer la línea");

            // Intenta convertir la entrada a un número. Si falla, pide de nuevo.
            match input.trim().parse() {
                Ok(num) => return num,
                Err(_) => {
                    println!("Entrada no válida. Por favor, introduce un número.");
                    continue;
                }
            };
        }
    }

    /// Imprime la matriz en la consola con un formato legible.
    fn imprimir(&self) {
        println!("\nMatriz de {}x{} creada:", self.filas, self.columnas);
        for fila in &self.datos {
            for valor in fila {
                print!("{} ", valor);
            }
            println!();
        }
    }
}

fn main() {
    // crera una matriz de mxn = que tecle el usuario ejemplo 2x3 dos filas tres columnas
    // mxn = [0 0 0]
    //       [0 0 0]

    // Creamos una instancia de nuestra Matriz usando el método que definimos.
    let matriz = Matriz::desde_entrada_usuario();

    // Usamos el método de la instancia para imprimirla.
    matriz.imprimir();
}
