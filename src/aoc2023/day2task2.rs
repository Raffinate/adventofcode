use std::cmp::max;

use crate::common::AocFlags;

#[derive(Clone)]
pub struct Day2Task2;

impl crate::common::AocTask for Day2Task2 {
    fn solve(&self, _: AocFlags) -> String {
        let input = include_str! {"resources/day2task2.txt"};
        return Day2Task2::solve(input.to_owned());
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Color {
    red: i32,
    green: i32,
    blue: i32,
}

impl Day2Task2 {
    fn solve(input: String) -> String {
        let lines = input.lines();

        let id_sum = lines
            .into_iter()
            .map(Day2Task2::parse_line_id_rgbs)
            .map(|s| {
                s.1.iter().fold(
                    Color {
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                    |r, c| Color {
                        red: max(r.red, c.red),
                        green: max(r.green, c.green),
                        blue: max(r.blue, c.blue),
                    },
                )
            })
            .map(|s| s.red * s.blue * s.green)
            .sum::<i32>();

        return format!("{}", id_sum);
    }

    fn parse_line_id_rgbs(l: &str) -> (i32, Vec<Color>) {
        let mut base_split = l.split(":");
        let game_id = base_split
            .next()
            .unwrap()
            .replace("Game ", "")
            .parse::<i32>()
            .unwrap();

        let revealed_strs = base_split.next().unwrap().split(";").collect::<Vec<_>>();
        let colors = revealed_strs
            .iter()
            .map(|s| Day2Task2::parse_reveal(s))
            .collect::<Vec<_>>();

        return (game_id, colors);
    }

    fn parse_reveal(l: &str) -> Color {
        let base_split: Vec<&str> = l.split(",").collect();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        macro_rules! parse_color {
            ($s:expr, $x:expr ) => {{
                if ($s.contains(stringify!($x))) {
                    $x = $s
                        .replace(stringify!($x), "")
                        .trim()
                        .parse::<i32>()
                        .unwrap();
                }
            }};
        }

        for s in base_split {
            parse_color!(s, red);
            parse_color!(s, green);
            parse_color!(s, blue);
        }

        return Color {
            red: red,
            green: green,
            blue: blue,
        };
    }
}
