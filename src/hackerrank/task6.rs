pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;
    let min_val = *a.iter().max().unwrap();
    let max_val = *b.iter().min().unwrap();

    for x in min_val..=max_val {
        let is_multiple_of_a = a.iter().all(|&val| x % val == 0);
        let divides_all_b = b.iter().all(|&val| val % x == 0);

        if is_multiple_of_a && divides_all_b {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }
}