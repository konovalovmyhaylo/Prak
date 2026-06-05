use std::io::{self, BufRead};

pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_sum = 0;
    let mut secondary_sum = 0;

    for i in 0..n {
        primary_sum += arr[i][i];
        secondary_sum += arr[i][n - 1 - i];
    }

    (primary_sum - secondary_sum).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let n: usize = iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::new();

    for _ in 0..n {
        if let Some(Ok(line)) = iterator.next() {
            let row: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            arr.push(row);
        }
    }

    let result = diagonal_difference(&arr);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference_example() {
        let arr = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];
        assert_eq!(diagonal_difference(&arr), 15);
    }

    #[test]
    fn test_single_element() {
        let arr = vec![vec![5]];
        assert_eq!(diagonal_difference(&arr), 0);
    }

    #[test]
    fn test_zeros() {
        let arr = vec![
            vec![0, 0],
            vec![0, 0],
        ];
        assert_eq!(diagonal_difference(&arr), 0);
    }

    #[test]
    fn test_negative_values() {
        let arr = vec![
            vec![-1, 1],
            vec![1, -1],
        ];
        assert_eq!(diagonal_difference(&arr), 4);
    }
}