fn suma(a: i32, b: i32) -> i32{
   a + b
}


fn main() {
    let s = suma(-1, -5);
    println!("SUMA: -> {}", s);
}


//creadno test pruebas unitarias 
#[test]
fn add_suma(){
    assert_eq!(suma(1,2), 3);
    assert_eq!(suma(0, 2), 2);
    assert_eq!(suma(1, 0), 1);
    assert_eq!(suma(-1, -2), -3);
}

#[test]
#[should_panic]
fn add_fails(){
    assert_eq!(suma(1, 3), 5);
}

#[test]
#[ignore = "no review by the Q:A team"]
fn add_negativas(){
    assert_eq!(suma(-1, -1), -2);
}

/*
-- Modulo de pruebas 
   La mayoría de las pruebas unitarias se incluyen en un 
   submódulo con el atributo #[cfg(test)].
-- El atributo cfg controla la compilación condicional y solo compilará el elemento 
   al que está asociado si el predicado es true. Cargo emite automáticamente la 
   marca de compilación test siempre que se ejecuta el comando $ cargo test, 
   por lo que el predicado siempre será true cuando se ejecuten las pruebas.
-- La declaración use super::*; es necesaria para que el código del módulo 
   add_function_tests pueda acceder a la función add en el módulo externo.
*/

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod add_function_tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 2), 7);
    }

    #[test]
    #[ignore]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4)
    }
}


