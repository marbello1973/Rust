struct Persona {
    first: String,
    middle: Option<String>,
    last: String
}

fn build_full_name(perona: &Persona) -> String{
    let mut full_name = String::new();
    full_name.push_str(&perona.first);
    full_name.push_str(" ");

    if let Some(middle) = &perona.middle{
        full_name.push_str(&middle);
        full_name.push_str(" ");
    }

    full_name.push_str(&perona.last);
    full_name
}

// Manejo de result
#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError>{
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    }else{
        Ok(dividend / divisor)
    }
}

fn main() {

    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
    
    let john = Persona {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Persona {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Persona {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };
    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
}
 
