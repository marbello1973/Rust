use std::fmt;

struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }        
    }
}

impl<T: fmt::Display> fmt::Display for Container<T> {
    fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
        write!(f, "Valor: {}", self.value)
    }
}

fn main() {
    assert_eq!(Container::new(42).value, 42);
    //assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(Container::new(String::from("Bar")).value, String::from("Bar"));
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));
}


/*
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

impl AsJson for Person {
    fn as_json(&self) -> String {
        format!(r#"{{ "type": "person", "name": "{}", "age": {}, "favorite_fruit": "{}"}}"#,
            self.name,
            self.age,
            self.favorite_fruit        
        )           
    }
}
*/


