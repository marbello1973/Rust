
pub fn count_letters(text: &str) -> usize{
    text.chars().filter(|ref c| c.is_alphabetic()).count()
}



