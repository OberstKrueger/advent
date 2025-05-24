use crate::Answer;

use std::collections::HashMap;

pub fn solution_2024_01(input: &str) -> Answer {
    let mut answer = Answer {
        first: Some(0),
        second: Some(0),
    };
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();
    let mut hash: HashMap<i64, i64> = HashMap::new();

    for line in input.lines() {
        let values: Vec<i64> = line
            .split_whitespace()
            .filter_map(|number| number.parse::<i64>().ok())
            .collect();

        if values.len() == 2 {
            list1.push(values[0]);
            list2.push(values[1]);
            *hash.entry(values[1]).or_insert(0) += 1;
        }
    }

    list1.sort();
    list2.sort();

    for (index, number) in list1.iter().enumerate() {
        let check: i64 = *hash.get(number).unwrap_or(&0);

        answer.first = Some(answer.first.unwrap_or(0) + (number - list2[index]).abs());
        answer.second = Some(answer.second.unwrap_or(0) + number * check);
    }

    answer
}

pub fn solution_2024_02(input: &str) -> Answer {
    let mut answer = Answer {
        first: Some(0),
        second: Some(0),
    };

    for line in input.lines() {
        let mut check: bool = false;
        let mut reports: Vec<Vec<i64>> = Vec::new();

        reports.push(
            line.split_whitespace()
                .filter_map(|number| number.parse::<i64>().ok())
                .collect(),
        );

        for index in 0..reports[0].len() {
            let mut copy = reports[0].clone();

            copy.remove(index);
            reports.push(copy);
        }

        for (index, report) in reports.iter().enumerate() {
            let mut errors: i64 = 0;

            if report.is_sorted() || report.is_sorted_by(|a, b| b <= a) {
                for window in report.windows(2) {
                    let difference: i64 = (window[1] - window[0]).abs();

                    if difference < 1 || difference > 3 {
                        errors += 1
                    }
                }
            } else {
                errors += 1;
            }

            match (index, errors) {
                (0, 0) => {
                    answer.first = Some(answer.first.unwrap_or(0) + 1);
                }
                (1.., 0) => check = true,
                _ => {}
            }
        }

        if check {
            answer.second = Some(answer.second.unwrap_or(0) + 1);
        }
    }

    answer
}

#[cfg(test)]
mod tests_2024 {
    use super::*;

    #[test]
    fn test_2024_01() {
        let input: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert_eq!(solution_2024_01(input).first, Some(11));
        assert_eq!(solution_2024_01(input).second, Some(31));
    }

    #[test]
    fn test_2024_02() {
        assert_eq!(solution_2024_02("7 6 4 2 1").first, Some(1));
        assert_eq!(solution_2024_02("1 2 7 8 9").first, Some(0));
        assert_eq!(solution_2024_02("9 7 6 2 1").first, Some(0));
        assert_eq!(solution_2024_02("1 3 2 4 5").first, Some(0));
        assert_eq!(solution_2024_02("1 3 6 7 9").first, Some(1));

        assert_eq!(solution_2024_02("7 6 4 2 1").second, Some(1));
        assert_eq!(solution_2024_02("1 2 7 8 9").second, Some(0));
        assert_eq!(solution_2024_02("9 7 6 2 1").second, Some(0));
        assert_eq!(solution_2024_02("1 3 2 4 5").second, Some(1));
        assert_eq!(solution_2024_02("1 3 6 7 9").second, Some(1));
    }
}
