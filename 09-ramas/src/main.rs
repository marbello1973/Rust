fn main() {

    // if
    let number = 6;

    if number < 5 {
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero!");
    }

    if number % 4 == 0 {
        println!("{number} divisible by 4");
    }else if number % 3 == 0{
        println!("{number} divisible by 3");
    } else if number % 2 == 0{
        println!("{number} divisible by 2");
    }else {
        println!("{number} not divisible by 4, 3, 2");
    }

    // asignando variables a expresion if

    let condition = true;
    let number_one = if condition { 5 } else { 6 };
    println!("The value of number {number_one}");

}
