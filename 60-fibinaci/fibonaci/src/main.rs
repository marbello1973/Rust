use std::vec;

fn main() {
    /* fibonacci
    let num_max: i32 = 10;
    let mut a = 0;
    let mut b = 1;

    for i in 0..num_max {
        match i {
            0 => println!("{}", a),
            1 => println!("{}", b),
            _ => {
                let c = a + b;
                println!("{}", c);
                a = b;
                b = c;
            }
        }
    }
    */

    println!("-------------------------------");
    let n = 6;
    let fill_char = '#';
    let matriz2 = construir_piramide_fold(n, fill_char);

    // Imprimir la matriz
    matriz2.iter().for_each(|fila| {
        fila.iter().for_each(|&c| print!("{}", c));
        println!();
    });
    println!("-------------------------------");

    // let mut matriz = vec![vec![' '; 2 * n - 1]; n];

    let mut matriz = vec![vec![' '; 2 * n - 1]; n];

    // Construir la pirámide en la matriz
    for i in 0..n {
        let centro = n - 1;
        for j in (centro - i)..=(centro + i) {
            matriz[i][j] = fill_char;
        }
    }

    // Imprimir la matriz
    for fila in matriz {
        for elemento in fila {
            print!("{}", elemento);
        }
        println!();
    }

    let matriz1: Vec<Vec<char>> = vec![vec![' '; n]; n];
    matriz1.iter().enumerate().for_each(|(i, fila)| {
        fila.iter().enumerate().for_each(|(j, &_col)| {
            print!("{}:{}", i, j);
        });
        // println!("FIL: {:?}", fila);
    });

    //println!("{:?}", matriz1);

    let vector = vec![1, 2, 3, 4, 5]
        .iter()
        .map(|x| x % 2 == 0)
        .filter(|&x| x)
        .collect::<Vec<bool>>();
    println!("{:?}", vector);

    // piramide
}

pub fn construir_piramide_fold(n: usize, fill_char: char) -> Vec<Vec<char>> {
    (0..n).fold(Vec::new(), |mut acc, i| {
        let fila = (0..2 * n - 1)
            .map(|j| {
                let centro = n - 1;
                if (centro - i..=centro + i).contains(&j) {
                    fill_char
                } else {
                    ' '
                }
            })
            .collect();
        acc.push(fila);
        acc
    })
}
