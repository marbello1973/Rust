
mod suma;
mod resta;

fn main() {
    let a_number = 10;
    let a_word = "Ten";
    println!("The first letter of the English alphabet is {} and the last letter is {}", a_number, a_word);

    let a_number = 15;
    println!("nuevo valor de a_number {}", a_number);

    let n = 5;

    let n = n + 5;

    let n = n * 2;

    println!("valor de N {}.", n);

    let b: u32 = 14;
    println!("valor de B {}.", b);

    let m = 4.0;
    let w: f32 = 5.0;

    println!("valores de M {} y W {}", m, w);
    
    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);
    
    
    let r = 1 < 4;
    println!("es 1 < 4 {}", r);

    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';
   
    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
    
    let tupla = ('E', 5i32, true);
    println!("tupla {} {} {}.", tupla.0, tupla.1, tupla.2);

    //const array:i32 = [i32; 6];
    //println!("array {}.", array);

    for i in 0..5{
        println!("{}", i);
    }

    let _varu8: u8 = 255;
    let _varu16: u16 = 65535;
    let _varu32: u32 = 4294967295;
    let _varu64: u64 = 18446744073709551615;
    let _varu128: u128 = 340282366920938463463374607431768211455;
    let _vari8: i8 = 127;
    let _vari16: i16 = 32767;
    let _vari32: i32 = 2147483647;
    let _vari64: i64 = 9223372036854775807;
    let _vari128: i128 = 170141183460469231731687303715884105727;
    let _varf32: f32 = 3.40282346638528859811704183484516925440;
    let _varf64: f64 = 1.797693134862315708145274237317043567981e+308;
    let _varbool: bool = true;
    let _varchar: char = 'a';
    let _varstr: &str = "hola";
    let _varstring: String = String::from("hola");
    let _varunit: () = ();
    let _vararray: [i32; 5] = [1, 2, 3, 4, 5];
    let _vartuple: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}, {}", _varstring,  _varstr);
    println!("{}, {}, {}", _vartuple.0, _vartuple.1, _vartuple.2);
    println!("{}, {}, {}, {}, {}", _varu8, _varu16, _varu32, _varu64, _varu128);


    let resultado = suma::sumar(5, 7);
    println!("resultado de la suma {}", resultado);
    
    let resultado = suma::sumar(10, 7);
    println!("resultado de la suma {}", resultado);

    let resultado = resta::restar(5, 7);
    println!("resultado de la resta {}", resultado);


}
