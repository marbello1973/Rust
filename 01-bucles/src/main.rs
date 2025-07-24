use std::collections::btree_map::Range;
use std::thread;
use std::time::Duration;
fn main() {
    // bucles loop , while, for

    let mut contador = 0;

    let result = loop {
        contador += 1;

        if contador == 9 {
            break contador * 2;
        }        

    };
    println!("the result contador: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if remaining == 2 {
                break 'counting_up;
            }
            remaining -= 1;           
        }
        count += 1;
        break;
    }

    println!("End count: {count}");

    // while
    let mut number_two = 5;

    while number_two != 0 {
        println!("number two: {number_two}");
        number_two -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];    
    let mut index = 0;
    while index <  5 {
        println!("the value is: {}", a[index]);        
        index += 1;
    }

    //for 
     for elementos in a {
        println!("the valorese de elementos de a: {elementos}")
     }

     for num in (1..10).rev(){        
        println!("conteo: {num}");
        thread::sleep(Duration::from_secs(2));
    }
    println!("¡tarea completada...!");
   
}

