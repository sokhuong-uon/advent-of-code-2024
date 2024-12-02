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
    for i in 1..levels.len() {
        let current_level = levels[i];
        let previous_level = levels[i - 1];

        if current_level.abs_diff(previous_level) > 3 || current_level == previous_level {
            return is_safe_with_dampener(&levels, i) || is_safe_with_dampener(&levels, i - 1);
        }

        if is_increasing == None {
            if current_level > previous_level {
                is_increasing = Some(true);
            } else {
                is_increasing = Some(false);
            }
        } else {
            if is_increasing.unwrap() && current_level < previous_level {
                return is_safe_with_dampener(&levels, i)
                    || is_safe_with_dampener(&levels, i - 1)
                    || if i >= 2 {
                        is_safe_with_dampener(&levels, i - 2)
                    } else {
                        false
                    };
            }
            if !is_increasing.unwrap() && current_level > previous_level {
                return is_safe_with_dampener(&levels, i)
                    || is_safe_with_dampener(&levels, i - 1)
                    || if i >= 2 {
                        is_safe_with_dampener(&levels, i - 2)
                    } else {
                        false
                    };
            }
        }
    }

    true
}

fn is_safe_with_dampener(levels: &Vec<u32>, i: usize) -> bool {
    let mut dampened_levels = levels.clone();
    dampened_levels.remove(i);

    let mut is_increasing: Option<bool> = None;

    for i in 1..dampened_levels.len() {
        let current_level = dampened_levels[i];
        let previous_level = dampened_levels[i - 1];

        if current_level.abs_diff(previous_level) > 3 || current_level == previous_level {
            return false;
        }

        if is_increasing == None {
            is_increasing = Some(current_level > previous_level);
        } else {
            if is_increasing.unwrap() && current_level < previous_level {
                return false;
            }
            if !is_increasing.unwrap() && current_level > previous_level {
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
        assert_eq!(is_safe("4 4 6 8"), true);
    }
}
