use crate::Answer;

pub fn solution_2015_01(input: &str) -> Answer {
    let mut answer = Answer::default();
    let mut found = false;

    for (index, character) in input.chars().enumerate() {
        match character {
            '(' => answer.first += 1,
            ')' => answer.first -= 1,
            _   => {}
        }

        if !found && answer.first == -1 {
            answer.second = index as i64 + 1;
            found = true;
        }
    }

    answer
}

pub fn solution_2015_02(input: &str) -> Answer {
    let mut answer = Answer::default();

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

            answer.first += side1 + total;
            answer.second += ribbon;
        }
    }

    answer
}

#[cfg(test)]
mod tests_2015 {
    use super::*;

    #[test]
    fn test_2015_01() {
        assert_eq!(solution_2015_01("(())").first, 0);
        assert_eq!(solution_2015_01("()()").first, 0);
        assert_eq!(solution_2015_01("(((").first, 3);
        assert_eq!(solution_2015_01("(()(()(").first, 3);
        assert_eq!(solution_2015_01("))(((((").first, 3);
        assert_eq!(solution_2015_01("())").first, -1);
        assert_eq!(solution_2015_01("))(").first, -1);
        assert_eq!(solution_2015_01(")))").first, -3);
        assert_eq!(solution_2015_01(")())())").first, -3);
        assert_eq!(solution_2015_01(")").second, 1);
        assert_eq!(solution_2015_01("()())").second, 5);
    }

    #[test]
    fn test_2015_02() {
        assert_eq!(solution_2015_02("2x3x4").first, 58);
        assert_eq!(solution_2015_02("1x1x10").first, 43);
        assert_eq!(solution_2015_02("2x3x4").second, 34);
        assert_eq!(solution_2015_02("1x1x10").second, 14);
    }
}
