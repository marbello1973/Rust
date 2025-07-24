use std::net::TcpListener;

struct Persona {
    name: String,
    lastname: String,
    dni: u32,
}

impl Persona {
    fn numero_dni(&self) -> Result<String, String> {
        let dni_str = self.dni.to_string();
        if dni_str.len() < 8 {
            Err("dni debe ser mayor a 8 - C".to_string())
        } else {
            Ok(dni_str)
        }
    }

    fn name_len(&self) -> Result<String, String> {
        if self.name.len() < 3 {
            Err("nombre un caracter".to_string())
        } else {
            Ok(self.name.clone())
        }
    }

    fn lastname_len(&self) -> Result<String, String> {
        if self.lastname.len() < 3 {
            Err(String::from("mas de 2"))
        } else {
            Ok(self.lastname.clone())
        }
    }
}

fn main() {
    let per = Persona {
        name: "da".to_string(),
        lastname: "apellido".to_string(),
        dni: 887871,
    };

    let p = per.numero_dni();
    let n = per.name_len();
    let l = per.lastname_len();
    println!("Name: {}, Last Name: {}, Dni: {}, Dni_Str: -> {:?}, Name len: -> {:?}, Last Name len: -> {:?}", 
        per.name, per.lastname, per.dni, p, n, l              
    );

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established! {}", stream.peer_addr().unwrap());
    }
}
