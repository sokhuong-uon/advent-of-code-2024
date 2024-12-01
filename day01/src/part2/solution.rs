use std::collections::HashMap;

use utils::get_file_content;

pub fn main() -> String {
    format!("{}", solution(&get_file_content("/day01/src/in.txt")))
}

fn solution(input: &str) -> u32 {
    let (left_vec, right_vec) = get_left_and_right_list(input);
    let right_vec = create_frequency_map(&right_vec);

    left_vec
        .iter()
        .map(|&num| {
            if let Some(&count) = right_vec.get(&num) {
                count * num
            } else {
                0
            }
        })
        .sum()
}

fn create_frequency_map(numbers: &[u32]) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    for &num in numbers {
        *map.entry(num).or_insert(0) += 1;
    }
    map
}

fn get_left_and_right_list(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        if numbers.len() == 2 {
            left.push(numbers[0]);
            right.push(numbers[1]);
        }
    }

    (left, right)
}
