use std::collections::HashMap;

fn main() {

    // CReamos un mapa hash vaco 
    let mut reviews: HashMap<String, String> = HashMap::new();
    // insertamos datos en el mapa HashMap
    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate"));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes"));
    reviews.insert(String::from("Programing in Rust"), String::from("Great examples"));

    // obtencion de un valor de la clave
    let book: &str ="Programing in Rust";
    println!("\nReviews for \'{}\': {:?}", book, reviews.get(book));

    //Eliminacion de un par clave 
    let obsolete:&str = "Ancient Roman History";
    println!("\n'{}\' removen. ", obsolete);
    reviews.remove(obsolete);
    //confirmamos que se elimino el elemento par clave 
    println!("\nReviews for \'{}\': {:?}", obsolete, reviews.get(obsolete));

    
    
}
