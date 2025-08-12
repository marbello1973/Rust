use wasm_bindgen::prelude::*;
struct Calculadora {
    a: i32,
    b: i32,
}

impl Calculadora {
    pub fn new(a: i32, b: i32) -> Self {
        Calculadora { a, b }
    }

    pub fn sumar(&self) -> i32 {
        self.a + self.b
    }

    pub fn restar(&self) -> i32 {
        self.a - self.b
    }

    pub fn mult(&self) -> i32 {
        self.a * self.b
    }
}

#[wasm_bindgen]
pub fn mul(a: i32, b: i32) -> i32 {
    let m: Calculadora = Calculadora::new(a, b);
    m.mult()
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    let sum = Calculadora::new(a, b);
    sum.sumar()
}

#[wasm_bindgen]
pub fn rest(a: i32, b: i32) -> i32 {
    let res = Calculadora::new(a, b);
    res.restar()
}
