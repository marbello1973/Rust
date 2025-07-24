fn main() {
    let formal = "Formal: Goodbay, see you!";
    let casual ="Casual: Bye, see you!";
    goodbay(formal);
    goodbay(casual);

    let num = 0;
    println!("{} divide by 5 = {}", num, divide_by_5(num));

    vector();
}

fn goodbay(message: &str) {   
    println!("{}", message);
}

fn divide_by_5(num: u32) -> u32 {
    //let result = num / 5;
    if num == 0 {       
        return 0;
    }
    num / 5
}

fn vector() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
    v.push(10);
    println!("{:?}", v);

    for i in &v {
        println!("{}", i);
    }
}



