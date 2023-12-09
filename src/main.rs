use std::{collections::HashMap, rc::Rc};

use clap::{self, Parser};
use common::AocTask;

use crate::common::AocFlags;

mod aoc2023;
mod common;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(clap::Parser, Clone, PartialEq, Eq, Hash)]
#[command(author, about, long_about = None)]
struct CommandLineArgs {
    year: i32,
    day: i32,
    puzzle: i32,
    #[arg(default_value = "false", long)]
    debug: bool,
    #[arg(default_value = "default", long, short)]
    algorithm: String,
}

// Given solution expression like
// aoc2023::day1task1::Day1Task1 {}
// will build CLI Args
fn build_cli_args_by_struct_expression(s: &str) -> CommandLineArgs {
    let mut split = s.split("::");
    let year = split
        .next()
        .expect(
            format!(
                "Expected fully qualified path to solution class like aoc2023::..., but got {}",
                s
            )
            .as_str(),
        )
        .replace("aoc", "")
        .parse::<i32>()
        .expect(format!("Crate name should look like aoc2023, but got {}", s).as_str());

    let module = split.next()
        .expect(format!("Expected fully qualified path to solution class like aoc2023::day1task1::..., but got {}", s).as_str());

    let binding = module.replace("task", "::");
    let mut day_puzzle_split = binding.split("::");
    let day = day_puzzle_split
        .next()
        .unwrap()
        .replace("day", "")
        .parse::<i32>()
        .expect(format!("Module name should look like dayXtaskX, but got {}", s).as_str());

    let puzzle = day_puzzle_split
        .next()
        .expect(format!("Module name should look like dayXtaskX, but got {}", s).as_str())
        .parse::<i32>()
        .expect(format!("Module name should look like dayXtaskX, but got {}", s).as_str());

    return CommandLineArgs {
        year: year,
        day: day,
        puzzle: puzzle,
        debug: false,
        algorithm: "default".to_owned(),
    };
}

macro_rules! aoc_task {
    ($s:expr) => {{
        (
            build_cli_args_by_struct_expression(stringify!($s)),
            Rc::new($s),
        )
    }};
}

fn main() {
    let cli = CommandLineArgs::parse();

    let solutions: Vec<(CommandLineArgs, Rc<dyn AocTask>)> = vec![
        aoc_task!(aoc2023::day1task1::Day1Task1 {}),
        aoc_task!(aoc2023::day1task2::Day1Task2 {}),
        aoc_task!(aoc2023::day2task1::Day2Task1 {}),
        aoc_task!(aoc2023::day2task2::Day2Task2 {}),
        aoc_task!(aoc2023::day3task1::Day3Task1 {}),
    ];

    let matcher: HashMap<CommandLineArgs, Rc<dyn AocTask>> = solutions.iter().cloned().collect();

    println!(
        "{}",
        matcher
            .get(&CommandLineArgs {
                year: cli.year,
                day: cli.day,
                puzzle: cli.puzzle,
                debug: false,
                algorithm: "default".to_owned(),
            })
            .expect(
                format!(
                    "No implementation registered for {}-{}-{}",
                    cli.year, cli.day, cli.puzzle
                )
                .as_str()
            )
            .solve(AocFlags { debug: cli.debug, algorithm: cli.algorithm})
    );
}
