
use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn main() {

    /*
    * En esta ejecución, el hilo principal imprimió primero, 
    * a pesar de que la instrucción de impresión del hilo creado aparece primero en el código. 
    * Y aunque le dijimos al hilo creado que imprimiera hasta que i sea 9, solo llegó a 5 antes 
    * de que el hilo principal se apagara.
    */
    thread::spawn(|| {
        for i in 1..5 {
            println!("el numero {i} en hilos de SPAWN ejecucion: ->"); 
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("Numero {i} sin hilos de MAIN ejecucion");
        thread::sleep(Duration::from_millis(1)  );
    }


    // esperan que los hilos terminen su ejecucion
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("el numero {i} en hilos de HANDLE SPAWN ejecucion: ->"); 
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    
     for i in 1..5 {
        println!("Numero {i} sin hilos de HANDLE MAIN ejecucion");
        thread::sleep(Duration::from_millis(1)  );
    }

    //handle.join().unwrap();
    

    /*
    * Para lograr la concurrencia mediante el envío de mensajes, la biblioteca estándar de Rust 
    * proporciona una implementación de canales. Un canal es un concepto de programación general 
    * por el cual se envían datos de un hilo a otro.
    */

    
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hola");
        tx.send(val).unwrap();
    });    

    let recibe = rx.recv().unwrap();
    println!("Dato recibido por hilo tx: {recibe}");


    //enviando mutiples valores
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("Hola,"),
            String::from("como"),
            String::from("vas"),
            String::from("con hilos")
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Mas"),
            String::from("hilos"),
            String::from("de"),
            String::from("ejecucion"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recibe in rx {
        println!("texto: -> {recibe}");
    }

}
