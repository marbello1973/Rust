fn main() {
    // Cambia este valor para más/menos discos
    let discos = 3;
    let torre_a: Vec<u32> = (1..=discos).rev().collect();
    let torre_b = Vec::new();
    let torre_c = Vec::new();

    println!("Estado inicial:");
    imprimir_torres(&torre_a, &torre_b, &torre_c);

    // recursividad
    hanoi(
        torre_a.len() as u32,
        &mut torre_a.clone(),
        &mut torre_c.clone(),
        &mut torre_b.clone(),
    );
}

fn hanoi(n: u32, origen: &mut Vec<u32>, destino: &mut Vec<u32>, auxiliar: &mut Vec<u32>) {
    if n > 0 {
        // Mover n-1 discos de origen a auxiliar
        hanoi(n - 1, origen, auxiliar, destino);

        // Mover el disco restante de origen a destino
        if let Some(disco) = origen.pop() {
            destino.push(disco);
            println!("\nMoviendo disco {}:", disco);
            imprimir_torres(origen, auxiliar, destino);
        }

        // Mover n-1 discos de auxiliar a destino
        hanoi(n - 1, auxiliar, destino, origen);
    }
}

fn imprimir_torres(a: &Vec<u32>, b: &Vec<u32>, c: &Vec<u32>) {
    println!("A: {:?}", a);
    println!("B: {:?}", b);
    println!("C: {:?}", c);
}
