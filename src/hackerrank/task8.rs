use std::io::{self, BufRead};

pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];
    
    for &bird in arr {
        if bird >= 1 && bird <= 5 {
            counts[bird as usize] += 1;
        }
    }
    
    let mut max_count = 0;
    let mut result_id = 0;
    
    for bird_id in 1..=5 {
        if counts[bird_id] > max_count {
            max_count = counts[bird_id];
            result_id = bird_id;
        }
    }
    
    result_id as i32
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let _ = iterator.next();

    if let Some(Ok(line)) = iterator.next() {
        let arr: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let result = migratory_birds(&arr);
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds_example1() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_migratory_birds_example2() {
        let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
        assert_eq!(migratory_birds(&arr), 3);
    }

    #[test]
    fn test_all_same() {
        let arr = vec![2, 2, 2, 2];
        assert_eq!(migratory_birds(&arr), 2);
    }

    #[test]
    fn test_tie_breaker() {
        let arr = vec![5, 5, 2, 2, 1];
        assert_eq!(migratory_birds(&arr), 2);
    }
}