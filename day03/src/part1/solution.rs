use regex::Regex;
use utils::get_file_content;

pub fn main() -> String {
    format!("{}", solution(&get_file_content("/day03/src/in.txt")))
}

fn solution(input: &str) -> u32 {
    let mut total = 0;
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for capture in pattern.captures_iter(input) {
        let x: u32 = capture[1].parse().unwrap();
        let y: u32 = capture[2].parse().unwrap();
        total += x * y;
    }
    total
}
