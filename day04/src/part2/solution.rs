use utils::get_file_content;

pub fn main() -> String {
    format!("{}", solution(&get_file_content("/day04/src/in.txt")))
}

fn solution(input: &str) -> u32 {
    let matrix = convert_input_to_matrix(input);

    matrix.iter().enumerate().fold(0, |acc, (row, line)| {
        let mut count = 0;

        for col in 0..line.len() {
            if matrix[row][col] == 'A' {
                if row == 0 || row == matrix.len() - 1 || col == 0 || col == line.len() - 1 {
                    continue;
                }

                if ((top_left(&matrix, row, col) == 'M' && bottom_right(&matrix, row, col) == 'S')
                    || (top_left(&matrix, row, col) == 'S'
                        && bottom_right(&matrix, row, col) == 'M'))
                    && ((top_right(&matrix, row, col) == 'M'
                        && bottom_left(&matrix, row, col) == 'S')
                        || (top_right(&matrix, row, col) == 'S'
                            && bottom_left(&matrix, row, col) == 'M'))
                {
                    count += 1;
                }
            }
        }

        acc + count
    })
}

fn top_right(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> char {
    matrix[row - 1][col + 1]
}

fn top_left(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> char {
    matrix[row - 1][col - 1]
}

fn bottom_right(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> char {
    matrix[row + 1][col + 1]
}

fn bottom_left(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> char {
    matrix[row + 1][col - 1]
}

fn convert_input_to_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
