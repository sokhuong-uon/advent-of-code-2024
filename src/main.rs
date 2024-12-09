use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Advent of Code solutions")]
struct Args {
    /// Positive integer in range [1,25]
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: Option<u8>,

    /// Positive integer in range [1,2]
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,
}

fn main() {
    let args = Args::parse();
    match (args.day, args.part) {
        (Some(day), Some(part)) => run_solution(day, part),
        _ => {
            eprintln!("Please provide both day and part numbers.");
            eprintln!("Usage: cargo run -- --day <DAY> --part <PART>");
            eprintln!("For more information, run: cargo run -- --help");
            process::exit(1);
        }
    }
}

fn run_solution(day: u8, part: u8) {
    match (day, part) {
        (1, 1) => println!("{}", day01::part1::solution::main()),
        (1, 2) => println!("{}", day01::part2::solution::main()),
        (2, 1) => println!("{}", day02::part1::solution::main()),
        (2, 2) => println!("{}", day02::part2::solution::main()),
        (3, 1) => println!("{}", day03::part1::solution::main()),
        (3, 2) => println!("{}", day03::part2::solution::main()),
        (4, 1) => println!("{}", day04::part1::solution::main()),
        _ => println!(
            "Solution for Day {} Part {} is not implemented yet.",
            day, part
        ),
    }
}
