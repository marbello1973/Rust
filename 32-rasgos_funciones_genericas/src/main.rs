trait AsJson {
    fn as_json(&self) -> String;
}

//escribir la misma función, pero con una sintaxis algo distinta, 
//indica explícitamente que T es un tipo genérico que debe implementar el rasgo AsJson:
//fn send_data_as_json<T: AsJson>(value: &T) { ... }
fn send_data_as_json(value: &impl AsJson){
    // add code here
    println!("Sending JSON data to server: ");
    println!("Data: {}", value.as_json());
    println!("Done!?\n");
}

//Podemos entonces declarar nuestros tipos e implementar el rasgo AsJson para ellos:
struct Person {
    name: String,
    age: u8,
    favorite_fruit: String,
}

struct Dog {
    name: String,
    color: String,
    likes_petting: bool,
}

struct Cat {
    name: String,
    sharp_claws: bool,
}

impl AsJson for Person {
    fn as_json(&self) -> String {
        format!(r#"{{ "type": "person", "name": "{}", "age": {}, "favorite_fruit": "{}"}}"#,
            self.name,
            self.age,
            self.favorite_fruit        
        )           
    }
}

impl AsJson for Dog {
    fn as_json(&self) -> String {
        format!(r#"{{ "type": "dog", "name": "{}", "color": {}, "likes_petting": "{}"}}"#,
            self.name,
            self.color,
            self.likes_petting        
        )           
    }
}

impl AsJson for Cat {
    fn as_json(&self) -> String {
        format!(r#"{{ "type": "cat", "name": "{}", "sharp_claws": "{}"}}"#,
            self.name,
            self.sharp_claws        
        )           
    }
}


fn main() {
    //declaramos una nueva persona 
    let laura = Person {
        name: String::from("laura"),
        age: 31,
        favorite_fruit: String::from("manzana"),
    };

    //declaramos una nuevo mascota
    let toben = Dog {
        name: String::from("toben"),
        color: String::from("violeta"),
        likes_petting: true,
    };

    let kitty = Cat {
        name: String::from("kitty"),
        sharp_claws: true,
    };

    send_data_as_json(&laura);
    send_data_as_json(&toben);
    send_data_as_json(&kitty);

    //println!("Laura: {:?}, Mascota: {:?}", p, m);
}

