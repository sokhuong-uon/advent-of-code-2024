use utils::get_file_content;

pub fn main() -> String {
    format!("{}", solution(&get_file_content("/day04/src/in.txt")))
}

fn solution(input: &str) -> u32 {
    let matrix = convert_input_to_matrix(input);

    matrix.iter().enumerate().fold(0, |acc, (row, line)| {
        let mut count = 0;

        for column in 0..line.len() {
            if line[column] == 'X' {
                if find_xmas_to_the_right(&matrix, row, column) {
                    count += 1;
                }

                if find_xmas_to_the_left(&matrix, row, column) {
                    count += 1;
                }
            }
        }

        acc + count
    })
}

fn convert_input_to_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_xmas_to_the_right(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    if column + 3 >= matrix[row].len() {
        return false;
    }

    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut found = true;

    for i in 1..4 {
        if matrix[row][column + i] != xmas[i] {
            found = false;
            break;
        }
    }

    found
}

fn find_xmas_to_the_left(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    if column < 3 {
        return false;
    }

    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut found = true;

    for i in 1..4 {
        if matrix[row][column - i] != xmas[i] {
            found = false;
            break;
        }
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_xmas_to_the_right() {
        let matrix = vec![
            vec!['M', 'A', 'S', 'A', 'M'],
            vec!['S', 'X', 'M', 'A', 'S'],
            vec!['M', 'S', 'A', 'S', 'X'],
            vec!['X', 'A', 'X', 'M', 'A'],
        ];
        assert_eq!(find_xmas_to_the_right(&matrix, 1, 1), true);
        assert_eq!(find_xmas_to_the_right(&matrix, 2, 4), false);

        assert_eq!(find_xmas_to_the_right(&matrix, 3, 0), false);
        assert_eq!(find_xmas_to_the_right(&matrix, 3, 2), false);
    }

    #[test]
    fn test_find_xmas_to_the_left() {
        let matrix = vec![
            vec!['M', 'X', 'S', 'A', 'M'],
            vec!['S', 'X', 'M', 'A', 'S'],
            vec!['M', 'S', 'A', 'M', 'X'],
            vec!['X', 'A', 'M', 'X', 'A'],
        ];
        assert_eq!(find_xmas_to_the_left(&matrix, 0, 1), false);
        assert_eq!(find_xmas_to_the_left(&matrix, 1, 1), false);

        assert_eq!(find_xmas_to_the_left(&matrix, 2, 4), true);

        assert_eq!(find_xmas_to_the_left(&matrix, 3, 0), false);
        assert_eq!(find_xmas_to_the_left(&matrix, 3, 3), false);
    }
}
