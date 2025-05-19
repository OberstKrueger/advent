use crate::Answer;

pub fn solution_2015_01(input: &str) -> Answer {
    let mut answer = Answer {
        first: Some(0),
        second: None
    };

    for (index, character) in input.chars().enumerate() {
        match character {
            '(' => answer.first = Some(answer.first.unwrap_or(0) + 1),
            ')' => answer.first = Some(answer.first.unwrap_or(0) - 1),
            _   => {}
        }

        if answer.second.is_none() && answer.first == Some(-1) {
            answer.second = Some(index as i64 + 1);
        }
    }

    answer
}

#[cfg(test)]
mod tests_2015 {
    use super::*;

    #[test]
    fn test_01() {
        assert_eq!(solution_2015_01("(())").first, Some(0));
        assert_eq!(solution_2015_01("()()").first, Some(0));
        assert_eq!(solution_2015_01("(((").first, Some(3));
        assert_eq!(solution_2015_01("(()(()(").first, Some(3));
        assert_eq!(solution_2015_01("))(((((").first, Some(3));
        assert_eq!(solution_2015_01("())").first, Some(-1));
        assert_eq!(solution_2015_01("))(").first, Some(-1));
        assert_eq!(solution_2015_01(")))").first, Some(-3));
        assert_eq!(solution_2015_01(")())())").first, Some(-3));

        assert_eq!(solution_2015_01(")").second, Some(1));
        assert_eq!(solution_2015_01("()())").second, Some(5));
    }
}
