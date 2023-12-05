use crate::common::AocFlags;

#[derive(Clone)]
pub struct Day1Task1;

impl crate::common::AocTask for Day1Task1 {
    fn solve(&self, _: AocFlags) -> String {
        let input = include_str! {"resources/day1task1.txt"};

        fn extract_digits(s: &str) -> Vec<char> {
            return s.chars().filter(|a| char::is_numeric(*a)).collect();
        }

        return String::from(
            input
                .lines()
                .map(|l| {
                    format!(
                        "{}{}",
                        extract_digits(l).iter().next().unwrap_or(&'0'),
                        extract_digits(l).iter().last().unwrap_or(&'0'),
                    )
                })
                .map(|n| n.parse::<i32>().unwrap())
                .sum::<i32>()
                .to_string(),
        );
    }
}
