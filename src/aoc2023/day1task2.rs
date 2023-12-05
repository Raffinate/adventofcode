//include_str!{"resources/day1task1.txt"}

use std::collections::HashMap;

use regex::Regex;

use crate::common::AocFlags;

#[derive(Clone)]
pub struct Day1Task2;

impl crate::common::AocTask for Day1Task2 {
    fn solve(&self, flags: AocFlags) -> String {
        let input = include_str! {"resources/day1task2.txt"};
        return Day1Task2::solve(input.to_owned(), flags.debug);
    }
}

impl Day1Task2 {
    fn solve(input: String, debug: bool) -> String {
        let decoder_def = [
            ("1", '1'),
            ("2", '2'),
            ("3", '3'),
            ("4", '4'),
            ("5", '5'),
            ("6", '6'),
            ("7", '7'),
            ("8", '8'),
            ("9", '9'),
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ];

        let decoder = decoder_def
            .iter()
            .map(|p| (p.0.to_owned(), p.1.to_owned()))
            .collect::<HashMap<String, char>>();

        let patterns = decoder.keys().cloned().collect::<Vec<String>>();

        let re_xs: Vec<Regex> = patterns
            .iter()
            .map(|pat| Regex::new(pat).unwrap())
            .collect();

        fn find_first_last(re_xs: Vec<Regex>, s: String) -> (String, String) {
            let mut first_idx = usize::MAX;
            let mut first_match = "".to_owned();

            let mut last_idx = usize::MIN;
            let mut last_match = "".to_owned();

            for re in re_xs {
                let mut match_iter = re.find_iter(&s).into_iter();
                while let Some(m) = match_iter.next() {
                    if m.start() < first_idx {
                        first_idx = m.start();
                        first_match = m.as_str().to_owned();
                    }
                    if m.end() > last_idx {
                        last_idx = m.end();
                        last_match = m.as_str().to_owned();
                    }
                }
            }

            return (first_match, last_match);
        }
        let extracted_numbers = input
            .lines()
            .map(|l| {
                format!(
                    "{}{}",
                    decoder
                        .get(find_first_last(re_xs.to_owned(), l.to_owned()).0.as_str())
                        .unwrap(),
                    decoder
                        .get(find_first_last(re_xs.to_owned(), l.to_owned()).1.as_str())
                        .unwrap(),
                )
            })
            .collect::<Vec<String>>();

        if debug {
            return extracted_numbers.join("\n");
        }

        return extracted_numbers
            .iter()
            .map(|n| n.parse::<i32>().unwrap())
            .sum::<i32>()
            .to_string();
    }
}

#[cfg(test)]
mod test {
    use super::Day1Task2;

    #[test]
    fn test_one_digit() {
        let result = Day1Task2::solve("1".to_owned(), false);
        assert_eq!("11", result);
    }

    #[test]
    fn test_two_digits() {
        let result = Day1Task2::solve("1asd2".to_owned(), false);
        assert_eq!("12", result);
    }

    #[test]
    fn test_two_digits_inside() {
        let result = Day1Task2::solve("asd1asd2asd".to_owned(), false);
        assert_eq!("12", result);
    }

    #[test]
    fn test_two_numbers_inside() {
        let result = Day1Task2::solve("asoned1asd2aninesd".to_owned(), false);
        assert_eq!("19", result);
    }

    #[test]
    fn test_number_21_overlap() {
        let result = Day1Task2::solve("twonea".to_owned(), false);
        assert_eq!("21", result);
    }

    #[test]
    fn test_number_18_overlap() {
        let result = Day1Task2::solve("oneight".to_owned(), false);
        assert_eq!("18", result);
    }

    #[test]
    fn test_number_38_overlap() {
        let result = Day1Task2::solve("threeight".to_owned(), false);
        assert_eq!("38", result);
    }

    #[test]
    fn test_number_58_overlap() {
        let result = Day1Task2::solve("fiveight".to_owned(), false);
        assert_eq!("58", result);
    }

    #[test]
    fn test_number_98_overlap() {
        let result = Day1Task2::solve("nineight".to_owned(), false);
        assert_eq!("98", result);
    }

    #[test]
    fn test_nultiline_sum() {
        let result = Day1Task2::solve("nineight\nasd2sdd".to_owned(), false);
        assert_eq!("120", result);
    }

    #[test]
    fn test_given_example() {
        let result = Day1Task2::solve(
            r#"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"#
                .to_owned(),
            false,
        );
        assert_eq!("281", result);
    }

    #[test]
    fn test_fourone417oneone() {
        let result = Day1Task2::solve("fourone417oneone".to_owned(), false);
        assert_eq!("41", result);
    }
}
