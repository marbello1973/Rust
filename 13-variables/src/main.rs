use std::io;
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");    
   

    // declaracion de constantes
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    let spaces = "    ";

    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");

    let s = suma(5, 10);    

    println!("The value of s is: {s}");

    // operaciones numericas 
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("The value of sum is: {sum}");
    println!("The value of difference is: {difference}");
    println!("The value of product is: {product}");
    println!("The value of quotient is: {quotient}");
    println!("The value of remainder is: {remainder}");


    // operaciones booleanas
    let t = true;
    let f: bool = false;
    println!("The value of t is: {t}");
    println!("The value of f is: {f}");

    // char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    // tuplas
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;    
    println!("The value of y is: {x} - {y} - {z} ");

    let one = tup.0;
    println!("The value of one is: {one}");

    //arrays
    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    let second = array[1];
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");

    adivinanzas();

    let number_one: u32 = 14;
    println!("the number is {}.", number_one);

    let number_two: &str = "14";
    println!("The numberTwo is string {}.", number_two);

    println!(" division de enteros y flotantes 9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2,  9.0 / 2.0);
    


}

fn suma(a: i32, b: i32) -> i32{
    return a + b;
}

fn adivinanzas(){
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin().read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
