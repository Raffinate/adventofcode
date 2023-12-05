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
}

fn main() {
    let cli = CommandLineArgs::parse();

    let solutions: Vec<(CommandLineArgs, Rc<dyn AocTask>)> = vec![
        (
            CommandLineArgs {
                year: 2023,
                day: 1,
                puzzle: 1,
                debug: false,
            },
            Rc::new(aoc2023::day1task1::Day1Task1 {}),
        ),
        (
            CommandLineArgs {
                year: 2023,
                day: 1,
                puzzle: 2,
                debug: false,
            },
            Rc::new(aoc2023::day1task2::Day1Task2 {}),
        ),
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
            })
            .unwrap()
            .solve(AocFlags { debug: cli.debug })
    );
}
