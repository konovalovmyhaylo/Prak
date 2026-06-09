pub fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = (n / 2) - (p / 2);

    std::cmp::min(from_front, from_back)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        assert_eq!(page_count(6, 2), 1);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(page_count(5, 4), 0);
    }

    #[test]
    fn test_even_end_page_exact() {
        assert_eq!(page_count(6, 5), 1);
    }

    #[test]
    fn test_odd_end_page_exact() {
        assert_eq!(page_count(7, 4), 1);
    }

    #[test]
    fn test_first_page() {
        assert_eq!(page_count(5, 1), 0);
    }

    #[test]
    fn test_last_page_even() {
        assert_eq!(page_count(6, 6), 0);
    }
}