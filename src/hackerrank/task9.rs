use std::io::{self, BufRead};

pub fn sock_merchant(n: i32, ar: &[i32]) -> i32 {
    let mut counts = [0; 101];
    let mut pairs = 0;

    for &sock in ar {
        if sock >= 1 && sock <= 100 {
            counts[sock as usize] += 1;
        }
    }

    for i in 1..=100 {
        pairs += counts[i] / 2;
    }

    pairs
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let _ = iterator.next();

    if let Some(Ok(line)) = iterator.next() {
        let ar: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let result = sock_merchant(ar.len() as i32, &ar);
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant_example1() {
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(9, &ar), 3);
    }

    #[test]
    fn test_sock_merchant_example2() {
        let ar = vec![1, 1, 3, 1, 2, 1, 3, 3, 3, 3];
        assert_eq!(sock_merchant(10, &ar), 4);
    }

    #[test]
    fn test_no_pairs() {
        let ar = vec![1, 2, 3, 4, 5];
        assert_eq!(sock_merchant(5, &ar), 0);
    }

    #[test]
    fn test_all_same() {
        let ar = vec![10, 10, 10, 10, 10];
        assert_eq!(sock_merchant(5, &ar), 2);
    }
}