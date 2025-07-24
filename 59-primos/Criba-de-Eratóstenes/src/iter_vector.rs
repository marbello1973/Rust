pub fn iter_vector(pr: Vec<bool>) -> Vec<i64> {
    pr.iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(i, _)| i as i64)
        .collect()
}
