pub fn is_even(num:i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_even () {
    assert!(is_even(2));
    }
    
    #[test]
    fn is_false_evem () {
            assert!(!is_even(1));
    }

}

fn main() {   
    println!("Is Evene: {}", !is_even(2));
}

