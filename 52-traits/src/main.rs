

fn main() {
    let x = 5;
    for i in 0..x {

        for _ in 0..( x - i - 1){
            print!(" ")
        }

        for _ in 0..(2 * i * 1){
            print!("*")
        }

        println!()
    } 
}