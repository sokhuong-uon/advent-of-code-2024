use regex::Regex;
use utils::get_file_content;

pub fn main() -> String {
    format!("{}", solution(&get_file_content("/day03/src/in.txt")))
}

fn solution(input: &str) -> u32 {
    let mut total = 0;

    let pattern = Regex::new(r"(?:mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut muls_enabled = true;

    for cap in pattern.captures_iter(input) {
        let full_match = &cap[0];
        match full_match {
            "do()" => {
                muls_enabled = true;
            }
            "don't()" => {
                muls_enabled = false;
            }
            _ if muls_enabled => {
                let x: u32 = cap[1].parse().unwrap();
                let y: u32 = cap[2].parse().unwrap();
                total += x * y;
            }
            _ => {}
        }
    }

    total
}
