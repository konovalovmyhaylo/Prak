pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade >= 38 {
            let next_multiple_of_5 = ((grade / 5) + 1) * 5;
            if next_multiple_of_5 - grade < 3 {
                return next_multiple_of_5;
            }
        }
        grade
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        let input = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&input), expected);
    }
}