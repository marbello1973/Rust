// strucutura para declarar las variables del rectangulo
#[derive(Debug)]
struct Rectangulo {
    width: u32,
    heigth: u32,
}

// metodo
impl Rectangulo {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }
    fn can_hold(&self, other: &Rectangulo) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangulo {
        width: dbg!(30 * scale),
        heigth: 50,
    };

    let rect2 = Rectangulo {
        width: dbg!(10 * scale),
        heigth: 40,
    };

    let rect3 = Rectangulo {
        width: 60,
        heigth: 45,
    };

    println!("area es de {} rectangulo", rect1.area());
    println!("area es de {} rectangulo", rect2.area());
    println!("area es de {} rectangulo", rect1.can_hold(&rect2));
    println!("area es de {} rectangulo", rect1.can_hold(&rect3));

    // forma de imprimir el debug
    println!("rect1 is: {rect1:?}");
    dbg!(&rect1);
}
