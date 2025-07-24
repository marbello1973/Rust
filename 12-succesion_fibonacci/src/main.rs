fn main() {

    // Fibonacci
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    println!("serie fibonacci:");
    
    for _ in 0..10 {
        c = a + b;       
        a = b;       
        b = c;       
        println!("{}", c);
    }

    // imprimir las letras dela cancion 'lod doce dias de navidad';
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh", "twelfth"];
    let gifts = ["A partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    for day in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", days[day]);
        for gift in (0..day + 1).rev() {
            if day > 0 && gift == 0 {
                print!("and ");
            }
            println!("{}", gifts[gift]);
        }
        println!();
    }

    
}

