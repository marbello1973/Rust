/*
Escritura de pruebas de documentación
Con Rust, puede ejecutar ejemplos de documentación como pruebas. 
La forma principal de documentar una biblioteca de Rust es mediante la anotación 
del código fuente con barras diagonales triples (///), lo que se conoce como 
comentarios de documentación. Los comentarios de documentación se escriben 
en Markdown y admiten bloques de código, de modo que estos bloques de código 
se compilan y se usan como pruebas.
Para probar esta característica, primero debe crear un nuevo proyecto de biblioteca.
*/

/// Generally, the first line is a brief summary describing the function.
///
/// The next lines present detailed documentation. 
/// Code blocks start with triple backticks. The code has an implicit `fn main()` inside and `extern crate <cratename>`,  
/// which means you can just start writing code.
/// 
/// let result = basic_math::add(2, 3);
/// assert_eq!(result, 5);
/// 
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
