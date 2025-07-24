use crate::is_prime::is_prime;

pub fn max_number() -> i64 {
    let mut max = 0;
    for i in 2..20 {
        if is_prime(i) {
            max = i;
        }
    }
    max
}
