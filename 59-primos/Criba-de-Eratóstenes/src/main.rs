/*
Como resultado se obtienen los números primos comprendidos
entre 2 y 20, y estos son: 2, 3, 5, 7, 11, 13, 17, 19.
*/

mod is_prime;
mod iter_vector;
mod max_number;

use iter_vector::iter_vector;
use max_number::max_number;

fn main() {
    let max = max_number();
    let mut vector = vec![true; (max + 1) as usize];
    vector[0] = false;
    vector[1] = false;
    let mut i = 2;
    loop {
        if i * i > max {
            break;
        }
        if vector[i as usize] {
            let mut j = i * i;
            while j <= max {
                vector[j as usize] = false;
                j += i;
            }
        }
        i += 1;
    }

    let vect = iter_vector(vector);

    println!("Números primos entre 2 y 20: {:?}", vect);
}
