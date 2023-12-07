
use crate::common::AocFlags;

#[derive(Clone)]
pub struct Day2Task1;

impl crate::common::AocTask for Day2Task1 {
    fn solve(&self, _: AocFlags) -> String {
        let input = include_str! {"resources/day2task1.txt"};
        return Day2Task1::solve(input.to_owned());
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Color {
    red: i32,
    green: i32,
    blue: i32,
}

impl Day2Task1 {
    fn solve(input: String) -> String {
        let lines = input.lines();

        let id_sum = lines
            .into_iter()
            .map(Day2Task1::parse_line_id_rgbs)
            .filter(|s| {
                s.1.iter()
                    .all(|c| c.red <= 12 && c.green <= 13 && c.blue <= 14)
            })
            .map(|s| s.0)
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
        let colors = revealed_strs.iter().map(|s| Day2Task1::parse_reveal(s)).collect::<Vec<_>>();

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
            // if s.contains("red") {
            //     red = s.replace("red", "").trim().parse::<i32>().unwrap();
            // } else if s.contains("blue") {
            // }
            // if s.contains("blue") {
            //     green = s.replace("blue", "").trim().parse::<i32>().unwrap();
            // }
            // if s.contains("green") {
            //     blue = s.replace("green", "").trim().parse::<i32>().unwrap();
            // }
        }

        return Color {
            red: red,
            green: green,
            blue: blue,
        };
    }
}
