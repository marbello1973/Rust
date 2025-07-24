// manjeo de memoria 
/*
fn mascota(){
    
        let mascot = String::from("toben");
        let toben = mascot;
        println!("{}", toben);
    
        // esto genera un error fuera del scope de ejejcucion del programa
        // porque esta fuera de las llaves {}
       // println!("{}", mascot);
    
}
*/


fn process(_input: u32){}

fn copias(_s: String){}

//prstando por referencia con el simbolo &
fn greet(){
    let greeting = String::from("Hola, Mundo");
    let greeting_reference = &greeting;
    println!("GREETING: {:?}", greeting_reference);
}

fn print_greeting(msg: &String){
    println!("print_greeting: {}", msg);
}

//mutacion de valores prestados, no compila porque la variable debe ser mutable
//pero al ponerla mut se puede cambiar la variable original, no seria un prestamo
fn change(_msg: &String){
    //msg.push_str("change mensaje");
}

// al ser mtable la variable, se pueden cambiar los valores originales de la variable
fn change_uno(text: &mut String){
    text.push_str(", mundo");
}


//prestamos 
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String{
    if x.len() > y.len(){
        x
    }else {
        y
    }
}

//Intentar transferir el valor de text contiene fuera del ambito
#[derive(Debug)]
struct Highligh<'doc>(&'doc str);

fn erase(_: String){}


fn main() {

    /*let mascota =String::from("toben");
    let toben = mascota;
    println!("mascota: {:?}", mascota);*/
    //let s = String::from("Hola, Mundo...!");
    let n = 1u32;
    //process(s);
    //process(s); //Error ownership of the value
    process(n);
    process(n);


    // al clonar se consume memoria y recuros esmejor tomar como referencia una variable 
    let p = String::from("Hola, Mundo");
    let p1 = copias(p.clone());
    let p2 = copias(p);
    println!("p1: {:?}, p2: {:?}", p1, p2);

    greet();

    let g = String::from("PRINT_GREETING");
    print_greeting(&g);
    print_greeting(&g);


    let chan = String::from("change reference");
    change(&chan);

    let mut texto = String::from("hola");
    change_uno(&mut texto);

    let magic1 = String::from("Abracadabra");
    let magic2 = String::from("Shazam");
    let result = longest_word(&magic1, &magic2);
    println!("longest world: {}", result);


    //transferie el texto
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highligh(&text[4..19]);
    let dog = Highligh(&text[35..43]);
    erase(text.clone());
    println!("{:?}", fox);
    println!("{:?}", dog);

}
