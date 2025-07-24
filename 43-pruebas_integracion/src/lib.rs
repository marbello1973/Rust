
/*
Escritura de pruebas de integración
Completado
100 XP
5 minutos
Las pruebas unitarias y de documentación proporcionan pruebas concisas y específicas. 
Pero también es una buena idea probar nuestro contenedor como un todo. Luego, 
podemos confirmar que los distintos elementos de código del contenedor funcionan 
juntos según lo previsto.

Para probar nuestro contenedor como un todo, podemos usar pruebas de integración. 
El conjunto de pruebas con Rust admite este tipo de prueba, que solo llama a las 
funciones que contiene la API pública de nuestra biblioteca. Podemos usar pruebas
de integración para comprobar cómo funciona nuestro código cuando otros lo usan.

La característica exclusiva de estas pruebas es que se encuentran en un directorio
y un archivo independientes, por lo que se pueden usar para probar externamente el 
código de la biblioteca. Al ejecutar pruebas de integración con Cargo, colóquelas 
en un directorio de pruebas. Cargo ejecuta cada archivo de origen en este directorio. 
Cree pruebas en el directorio del proyecto, en el mismo nivel que el directorio src.

Vamos a escribir algunas pruebas de integración mediante la creación de un pequeño 
proyecto. Ejecute los siguientes comandos en el terminal:

*/

pub struct Pizza {
    pub topping: String,
    pub inches: u8,
}

impl Pizza {
    pub fn pepperoni(inches: u8) -> Self {
        Pizza::bake("pepperoni", inches)
    }

    pub fn mozzarella(inches: u8) -> Self {
        Pizza::bake("mozzarella", inches)
    }

    fn bake(topping: &str, inches: u8) -> Self {
        Pizza {
            topping: String::from(topping),
            inches,
        }
    }
}
/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
