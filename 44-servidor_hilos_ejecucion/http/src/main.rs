use http::ThreadPool;
use std::net::TcpListener;
mod handle_connection;
use std::io::Result;


fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(5);

    for stream in listener.incoming().take(5) {        
        let stream = stream?;
        let stream_clone = stream.try_clone()?;
        pool.execute(move || { 
           if let Err(e) = handle_connection::handle(stream_clone){
                eprintln!("Error: {}", e);
           }
        });             
    }

    println!("Shitting down...!");

    Ok(())
    
}


/*FUNCION ORIGINAL
fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(10);

    for stream in listener.incoming().take(5) {        
        let stream = stream?;
        pool.execute(|| { 
            handle_connection::handle(stream);
        });
    }
    println!("Shitting down...!");

    Ok(())
    
}
*/