//use core_1::adivina_numero;
//use core_2::app_consola;
//use core_3::figuras_geometricas;
//use core_4::kangaroo;
//use core_4::get_percentage_rounds;
//use core_4::get_totall_x;

fn main() {
    /* core-1
    //libreria para el juego adivinar un numero
    //adivina_numero();
     */

    /* core-2
    //dos pciones usar Result para manejar el error o no solo asignar a un barra al piso Ej: let _ = app_consola();
    //Opcion 1 sin manejaor de error
    // let _ = app_consola();

    //Opcion 2 majenar el Result<(), String>
    let app: Result<(), String> = match app_consola() {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error {}", e);
            Err(e)
        }
    };

    if let Err(e) = app {
        println!("Error {e}");
    }
    */

    /* core-3
    //simple ejercicio de figuras geometricas en consola
    figuras_geometricas();
    */

    /* core-4
    kangaroo(0, 2, 5, 3);
    */

    /* core-4
    let percentages = vec![
        0.0, 0.15, 0.25, 0.35, 0.45, 0.55, 0.65, 0.75, 0.85, 0.95, 1.0,
        ];

        for percentage in percentages {
            println!("{}: {}", percentage, get_percentage_rounds(percentage));
            }
    */

    /* core-4
    let a: &[i32] = &[2, 3];
    let b: &[i32] = &[2, 4];
    let c: &[i32] = &[16, 32, 96];

    let resultado_ab: i32 = get_totall_x(&a, &b);
    let resultado_ac: i32 = get_totall_x(&a, &c);

    println!("Resultado AB: {resultado_ab}");
    println!("Resultado AC {resultado_ac}");
    */
}
