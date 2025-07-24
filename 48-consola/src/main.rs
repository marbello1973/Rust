use std::fs;
use std::io;
use std::path::PathBuf;

fn main() {
    let mut current_dir = std::env::current_dir().expect("No se pudo obtener el directorio actual");

    loop {
        // Mostrar el contenido del directorio actual
        println!("\nDirectorio actual: {}", current_dir.display());
        let entries = fs::read_dir(&current_dir)
            .expect("No se pudo leer el directorio")
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect::<Vec<_>>();

        for (index, entry) in entries.iter().enumerate() {
            println!("{}: {}", index, entry);
        }

        // Pedir al usuario que elija una opción
        println!("\nSelecciona un índice para navegar, '..' para subir, o 'q' para salir:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");
        let input = input.trim();

        if input == "q" {
            println!("Saliendo...");
            break;
        } else if input == ".." {
            if let Some(parent) = current_dir.parent() {
                current_dir = parent.to_path_buf();
            } else {
                println!("No hay un directorio superior.");
            }
        } else if let Ok(index) = input.parse::<usize>() {
            if let Some(selected) = entries.get(index) {
                let new_path = current_dir.join(selected);
                if new_path.is_dir() {
                    current_dir = new_path;
                } else {
                    println!("La ruta seleccionada no es un directorio.");
                }
            } else {
                println!("Índice inválido.");
            }
        } else {
            println!("Entrada no válida.");
        }
    }
}

