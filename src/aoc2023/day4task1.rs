use crate::common::AocFlags;

#[derive(Clone)]
pub struct Day4Task1;

impl crate::common::AocTask for Day4Task1 {
    fn solve(&self, _: AocFlags) -> String {
        let input = include_str! {"resources/day4task1.txt"};
        return solution::solve(input);
    }
}

mod solution {
    use std::collections::HashSet;

    pub fn solve(input: &str) -> String {
        return input.lines().map(row_value).sum::<i32>().to_string();
    }

    fn row_value(line: &str) -> i32 {
        let tokens = line
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .split('|')
            .collect::<Vec<_>>();

        let win_numbers = tokens[0]
            .split_whitespace()
            .map(|t| {
                t.parse::<i32>()
                    .expect(format!("INVALID NUMBER!: {}", t).as_str())
            })
            .collect::<HashSet<_>>();

        let actual_numbers = tokens[1]
            .split_whitespace()
            .map(|t| {
                t.parse::<i32>()
                    .expect(format!("INVALID NUMBER!!: {}", t).as_str())
            })
            .collect::<Vec<_>>();

        let matches = actual_numbers
            .iter()
            .filter(|n| win_numbers.contains(n))
            .count();

        if matches <= 0 {
            return 0;
        }

        return 2_i32.pow((matches - 1) as u32);
    }
}

#[cfg(test)]
mod test {
    use super::solution::solve;

    #[test]
    fn test_example() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        let result = solve(input);
        assert_eq!("13", result);
    }
}
