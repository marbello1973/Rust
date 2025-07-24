use figlet_rs::FIGfont;
use std::io;

pub fn app_consola() -> Result<(), String> {
    let mut texto: String = String::new();
    let mut symbolo: String = String::new();

    println!("Ingrse el texto");

    match io::stdin().read_line(&mut texto) {
        Ok(_) => {}
        Err(e) => {
            println!("Ingrese valor valido {}", e);
            return Err("Error".to_string());
        }
    };

    println!("Ingrese el symbolo");
    io::stdin().read_line(&mut symbolo).expect("Error");
    let symbolo = symbolo.trim();

    if symbolo.chars().count() != 1 {
        eprintln!("Error: Debe ingresar un solo caracter");
        return Err("Error".to_string());
    }

    let symbolo = symbolo.chars().next().unwrap();

    let resultado = generar_banner(&texto, symbolo);
    println!("{}", resultado);

    Ok(())
}

pub fn generar_banner(texto: &str, symbolo: char) -> String {
    let stand_font = FIGfont::standard().unwrap();
    let figura = stand_font.convert(texto);

    if let Some(ascii_art) = figura {
        let mut result = String::new();
        for line in ascii_art.to_string().lines() {
            let transformar_linea: String = line
                .chars()
                .map(|c| {
                    if c.is_ascii_punctuation() || c.is_ascii_digit() {
                        symbolo
                    } else {
                        c
                    }
                })
                .collect();
            result.push_str(&transformar_linea);
            result.push('\n');
        }
        result
    } else {
        eprintln!("No se genero arte ASCII.");
        String::new()
    }
}
