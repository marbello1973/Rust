pub fn count_numbers(text: &str) -> usize {
   text.chars().filter(|ref c| c.is_numeric()).count()
}

