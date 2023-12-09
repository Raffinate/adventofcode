use crate::common::AocFlags;

#[derive(Clone)]
pub struct Day3Task2;

impl crate::common::AocTask for Day3Task2 {
    fn solve(&self, flags: AocFlags) -> String {
        let input = include_str! {"resources/day3task2.txt"};
        return solution::solve(input, flags.debug);
    }
}

mod solution {
    use std::collections::{HashMap, HashSet};

    struct ParserState {
        number: Vec<char>,
        is_part_number: bool,
    }

    impl ParserState {
        fn new() -> ParserState {
            return ParserState {
                number: Vec::new(),
                is_part_number: false,
            };
        }

        fn is_empty(&self) -> bool {
            return self.number.is_empty();
        }
    }

    impl ToString for ParserState {
        fn to_string(&self) -> String {
            return self.number.to_owned().into_iter().collect();
        }
    }

    pub fn solve(input: &str, debug: bool) -> String {
        let line_size = input.lines().next().unwrap().chars().count();
        let lines_count = input.lines().count();

        let mut chars: Vec<Vec<char>> = Vec::with_capacity(lines_count);

        for s in input.lines() {
            let mut ln: Vec<char> = Vec::with_capacity(s.chars().count());

            for c in s.chars() {
                ln.push(c);
            }

            chars.push(ln);
        }

        let mut numbers: Vec<String> = Vec::new();
        let mut gears: HashMap<(usize, usize), HashSet<usize>> = HashMap::new();

        for i in 0..lines_count {
            let mut state = ParserState::new();
            for j in 0..line_size {
                let c = chars[i][j];
                if c.is_ascii_digit() {
                    state.number.push(c);
                } else {
                    if state.is_part_number {
                        numbers.push(state.to_string());
                    }
                    state = ParserState::new();
                }
                if !state.is_empty() {
                    for di in -1..=1 {
                        for dj in -1..=1 {
                            let ii: i32 = i32::try_from(i).unwrap() + di;
                            let jj: i32 = i32::try_from(j).unwrap() + dj;
                            if ii < 0 || jj < 0 {
                                continue;
                            }

                            let cc = chars
                                .get(ii as usize)
                                .and_then(|l| l.get(jj as usize))
                                .unwrap_or(&'.');

                            if cc.to_owned() == '.' || cc.is_ascii_digit() {
                                continue;
                            } else {
                                state.is_part_number = true;
                                if cc.to_owned() == '*' {
                                    let gear_coords = (ii as usize, jj as usize);
                                    if !gears.contains_key(&gear_coords) {
                                        gears.insert(gear_coords, HashSet::<usize>::new());
                                    }

                                    let gear = gears.get_mut(&gear_coords).unwrap();

                                    gear.insert(numbers.len());
                                }
                            }
                        }
                    }
                }
            }
            if state.is_part_number {
                numbers.push(state.to_string());
            }
        }

        let parsed_numbers = numbers
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if debug {
            return numbers.join(",");
        }

        return gears
            .iter()
            .map(|g| g.1)
            .filter(|g| g.len() == 2)
            .map(|g| g.iter().cloned().collect::<Vec<_>>())
            .map(|g| {
                parsed_numbers.get(g.get(0).unwrap().to_owned()).unwrap()
                    * parsed_numbers.get(g.get(1).unwrap().to_owned()).unwrap()
            })
            .sum::<i32>()
            .to_string();
    }
}

#[cfg(test)]
mod test {
    use super::solution::solve;

    #[test]
    fn test_example() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let result = solve(input, false);
        assert_eq!("467835", result);
    }

    #[test]
    fn test_beginning_end() {
        let input = r#"35*633"#;
        let result = solve(input, false);
        assert_eq!("22155", result);
    }

    #[test]
    fn test_diagonal() {
        let input = r#"35....
..*...
...633"#;
        let result = solve(input, false);
        assert_eq!("22155", result);
    }

    #[test]
    fn test_new_line_break_number() {
        let input = "*1\n1.";
        let result = solve(input, false);
        assert_eq!("1", result);
    }
}
