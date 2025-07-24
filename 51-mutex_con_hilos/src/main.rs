use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    let contador = Arc::new(Mutex::new(0));

    let mut handler = vec![];
    
    for _ in 0..10 {
        
        let contador = Arc::clone(&contador);
        
        let handle = thread::spawn(move || {

            let mut num = contador.lock().unwrap();

            *num += 1;
        });

        handler.push(handle);
    }

    for handle in handler {
        handle.join().unwrap()
    }
    
    println!("Resultado:  {}", *contador.lock().unwrap());
}
