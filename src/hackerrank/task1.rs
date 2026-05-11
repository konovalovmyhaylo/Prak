pub fn solve_me_first(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(solve_me_first(10, 5), 15);
    }
}