use utils::get_file_content;

pub fn main() -> String {
    format!("{}", solution(&get_file_content("/day02/src/in.txt")))
}

fn solution(input: &str) -> u32 {
    let mut count = 0;
    input.lines().for_each(|line| {
        if is_safe(line) {
            count += 1;
        }
    });
    count
}

fn is_safe(line: &str) -> bool {
    let levels: Vec<u32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut is_increasing: Option<bool> = None;
    for idx in 1..levels.len() {
        if levels[idx].abs_diff(levels[idx - 1]) > 3 || levels[idx] == levels[idx - 1] {
            return false;
        }
        if is_increasing == None {
            if levels[idx] > levels[idx - 1] {
                is_increasing = Some(true);
            } else {
                is_increasing = Some(false);
            }
        } else {
            if is_increasing.unwrap() && levels[idx] < levels[idx - 1] {
                return false;
            }
            if !is_increasing.unwrap() && levels[idx] > levels[idx - 1] {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        assert_eq!(is_safe("5 1 9 5"), false);
        assert_eq!(is_safe("7 5 3"), true);
        assert_eq!(is_safe("2 4 6 8"), true);
        assert_eq!(is_safe("4 4 6 8"), false);
    }
}
