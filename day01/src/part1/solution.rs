use utils::get_file_content;

pub fn main() -> String {
    format!("{}", solution(&get_file_content("/day01/src/in.txt")))
}

fn solution(input: &str) -> u32 {
    let (mut left_vec, mut right_vec) = get_left_and_right_list(input);

    sort_vector(&mut left_vec);
    sort_vector(&mut right_vec);

    calculate_diff_sum(&left_vec, &right_vec)
}

fn calculate_diff_sum(left: &[u32], right: &[u32]) -> u32 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum()
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

fn sort_vector(vector: &mut Vec<u32>) {
    vector.sort();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_collector() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let (left, right) = get_left_and_right_list(input);

        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_sort_vector() {
        let mut vector = vec![3, 4, 2, 1, 3, 3];
        sort_vector(&mut vector);
        assert_eq!(vector, vec![1, 2, 3, 3, 3, 4]);
    }
}
