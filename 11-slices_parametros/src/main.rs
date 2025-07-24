fn main() {
    let my_string = String::from("Hola Mundo");
    let word = first_word(&my_string);

    println!("Hello, world! {word}");

    // Otros slices vectoriales
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice = &a[1..6];

    println!("{:?}", slice)
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
