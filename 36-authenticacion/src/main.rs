mod authenticacion;

use regex::Regex;

fn main() {
    
    let mut user = authenticacion::User::new("David", "secreet");
    
    println!("Username is: {}", user.get_username());

    user.set_password("nuevo-password");

    //Usando regex despues de instalado en dependencias
    let regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("REGEX: {}", regex.is_match("2024-01-01"));
}
