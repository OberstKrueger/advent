use crate::Answer;

pub fn solution_2015_01(input: &str) -> Answer {
    let mut answer = Answer {
        first: Some(0),
        second: None,
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

pub fn solution_2015_02(input: &str) -> Answer {
    let mut answer = Answer {
        first: Some(0),
        second: None,
    };

    for line in input.lines() {
        let mut dimensions: Vec<i64> = line
            .split('x')
            .filter_map(|number| number.parse::<i64>().ok())
            .collect();

        if dimensions.len() == 3 {
            dimensions.sort();

            let side1 = dimensions[0] * dimensions[1];
            let side2 = dimensions[0] * dimensions[2];
            let side3 = dimensions[1] * dimensions[2];

            let total = 2 * (side1 + side2 + side3);

            let ribbon = 2 * dimensions[0] + 2 * dimensions[1] + dimensions.iter().product::<i64>();

            answer.first = Some(answer.first.unwrap_or(0) + side1 + total);
            answer.second = Some(answer.second.unwrap_or(0) + ribbon);
        }
    }

    answer
}

#[cfg(test)]
mod tests_2015 {
    use super::*;

    #[test]
    fn test_2015_01() {
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

    #[test]
    fn test_2015_02() {
        assert_eq!(solution_2015_02("2x3x4").first, Some(58));
        assert_eq!(solution_2015_02("1x1x10").first, Some(43));
        assert_eq!(solution_2015_02("2x3x4").second, Some(34));
        assert_eq!(solution_2015_02("1x1x10").second, Some(14));
    }
}
