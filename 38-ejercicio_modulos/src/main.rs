mod letters;
mod numbers;

fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = letters::count_letters(text);
    let number_of_numbers = numbers::count_numbers(text);
    (number_of_letters, number_of_numbers)
    //println!("Numeros: {:?}", number_of_letters);
    //println!("Num and Num: {:?}", number_of_numbers);
}

fn main() {
    assert_eq!(count_letters_and_numbers("221B Baker Street"), (12, 3));
    assert_eq!(count_letters_and_numbers("711 Maple Street"), (11, 3));
    assert_eq!(count_letters_and_numbers("4 Privet Drive"), (11, 1));
}
