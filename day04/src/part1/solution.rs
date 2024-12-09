use utils::get_file_content;

pub fn main() -> String {
    format!("{}", solution(&get_file_content("/day04/src/in.txt")))
}

fn solution(input: &str) -> u32 {
    let matrix = convert_input_to_matrix(input);

    matrix.iter().enumerate().fold(0, |acc, (row, _)| {
        let count = check_xmases(&matrix, row);

        acc + count
    })
}

fn check_xmases(matrix: &Vec<Vec<char>>, row: usize) -> u32 {
    let checks = [
        is_xmas_right,
        is_xmas_left,
        is_xmas_up,
        is_xmas_down,
        is_xmas_first_quadrant,
        is_xmas_second_quadrant,
        is_xmas_third_quadrant,
        is_xmas_fourth_quadrant,
    ];

    let mut count = 0;

    for column in 0..matrix[row].len() {
        if matrix[row][column] == 'X' {
            count += checks
                .iter()
                .filter(|&check| check(&matrix, row, column))
                .count();
        }
    }

    count as u32
}

fn convert_input_to_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_xmas_right(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
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

fn is_xmas_left(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
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

fn is_xmas_up(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    if row < 3 {
        return false;
    }

    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut found = true;

    for i in 1..4 {
        if matrix[row - i][column] != xmas[i] {
            found = false;
            break;
        }
    }

    found
}

fn is_xmas_down(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    if row + 3 >= matrix.len() {
        return false;
    }

    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut found = true;

    for i in 1..4 {
        if matrix[row + i][column] != xmas[i] {
            found = false;
            break;
        }
    }

    found
}

fn is_xmas_first_quadrant(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    if row < 3 {
        return false;
    }

    if column + 3 >= matrix[row].len() {
        return false;
    }

    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut found = true;

    for i in 1..4 {
        if matrix[row - i][column + i] != xmas[i] {
            found = false;
            break;
        }
    }

    found
}

fn is_xmas_second_quadrant(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    if column < 3 || row < 3 {
        return false;
    }

    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut found = true;

    for i in 1..4 {
        if matrix[row - i][column - i] != xmas[i] {
            found = false;
            break;
        }
    }

    found
}

fn is_xmas_third_quadrant(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    if column < 3 {
        return false;
    }

    if row + 3 >= matrix.len() {
        return false;
    }

    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut found = true;

    for i in 1..4 {
        if matrix[row + i][column - i] != xmas[i] {
            found = false;
            break;
        }
    }

    found
}

fn is_xmas_fourth_quadrant(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    if row + 3 >= matrix.len() || column + 3 >= matrix[row].len() {
        return false;
    }

    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut found = true;

    for i in 1..4 {
        if matrix[row + i][column + i] != xmas[i] {
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
    fn test_is_xmas_right() {
        let matrix = vec![
            vec!['M', 'A', 'S', 'A', 'M'],
            vec!['S', 'X', 'M', 'A', 'S'],
            vec!['M', 'S', 'A', 'S', 'X'],
            vec!['X', 'A', 'X', 'M', 'A'],
        ];
        assert_eq!(is_xmas_right(&matrix, 1, 1), true);
        assert_eq!(is_xmas_right(&matrix, 2, 4), false);

        assert_eq!(is_xmas_right(&matrix, 3, 0), false);
        assert_eq!(is_xmas_right(&matrix, 3, 2), false);
    }

    #[test]
    fn test_is_xmas_left() {
        let matrix = vec![
            vec!['M', 'X', 'S', 'A', 'M'],
            vec!['S', 'X', 'M', 'A', 'S'],
            vec!['M', 'S', 'A', 'M', 'X'],
            vec!['X', 'A', 'M', 'X', 'A'],
        ];
        assert_eq!(is_xmas_left(&matrix, 0, 1), false);
        assert_eq!(is_xmas_left(&matrix, 1, 1), false);

        assert_eq!(is_xmas_left(&matrix, 2, 4), true);

        assert_eq!(is_xmas_left(&matrix, 3, 0), false);
        assert_eq!(is_xmas_left(&matrix, 3, 3), false);
    }

    #[test]
    fn test_is_xmas_up() {
        let matrix = vec![
            vec!['S', 'X', 'S', 'A', 'M'],
            vec!['A', 'X', 'M', 'A', 'S'],
            vec!['M', 'A', 'X', 'M', 'X'],
            vec!['X', 'M', 'M', 'X', 'A'],
            vec!['X', 'X', 'X', 'M', 'A'],
        ];
        assert_eq!(is_xmas_up(&matrix, 3, 0), true);
        assert_eq!(is_xmas_up(&matrix, 0, 1), false);
        assert_eq!(is_xmas_up(&matrix, 1, 1), false);

        assert_eq!(is_xmas_up(&matrix, 4, 2), false);

        assert_eq!(is_xmas_up(&matrix, 3, 3), false);
    }

    #[test]
    fn test_is_xmas_down() {
        let matrix = vec![
            vec!['S', 'X', 'S', 'A', 'M'],
            vec!['A', 'M', 'M', 'X', 'S'],
            vec!['M', 'A', 'X', 'M', 'X'],
            vec!['X', 'S', 'M', 'X', 'M'],
            vec!['X', 'X', 'X', 'M', 'A'],
        ];
        assert_eq!(is_xmas_down(&matrix, 3, 0), false);
        assert_eq!(is_xmas_down(&matrix, 4, 0), false);

        assert_eq!(is_xmas_down(&matrix, 0, 1), true);

        assert_eq!(is_xmas_down(&matrix, 2, 2), false);
        assert_eq!(is_xmas_down(&matrix, 4, 2), false);

        assert_eq!(is_xmas_down(&matrix, 2, 4), false);
    }

    #[test]
    fn test_is_xmas_first_quadrant() {
        let matrix = vec![
            vec!['S', 'X', 'S', 'S', 'M'],
            vec!['X', 'M', 'A', 'X', 'X'],
            vec!['M', 'M', 'X', 'A', 'A'],
            vec!['X', 'S', 'M', 'M', 'M'],
            vec!['X', 'X', 'X', 'M', 'X'],
        ];
        assert_eq!(is_xmas_first_quadrant(&matrix, 1, 0), false);
        assert_eq!(is_xmas_first_quadrant(&matrix, 0, 1), false);

        assert_eq!(is_xmas_first_quadrant(&matrix, 4, 2), false);
        assert_eq!(is_xmas_first_quadrant(&matrix, 4, 4), false);

        assert_eq!(is_xmas_first_quadrant(&matrix, 3, 0), true);
        assert_eq!(is_xmas_first_quadrant(&matrix, 4, 1), false);
    }

    #[test]
    fn test_is_xmas_second_quadrant() {
        let matrix = vec![
            vec!['S', 'X', 'S', 'S', 'M'],
            vec!['X', 'S', 'A', 'X', 'X'],
            vec!['A', 'M', 'A', 'A', 'A'],
            vec!['X', 'M', 'M', 'M', 'M'],
            vec!['X', 'X', 'X', 'M', 'X'],
        ];
        assert_eq!(is_xmas_second_quadrant(&matrix, 1, 0), false);
        assert_eq!(is_xmas_second_quadrant(&matrix, 0, 1), false);
        assert_eq!(is_xmas_second_quadrant(&matrix, 0, 4), false);

        assert_eq!(is_xmas_second_quadrant(&matrix, 4, 2), false);
        assert_eq!(is_xmas_second_quadrant(&matrix, 4, 4), true);
    }

    #[test]
    fn test_is_xmas_third_quadrant() {
        let matrix = vec![
            vec!['S', 'X', 'S', 'S', 'X'],
            vec!['X', 'S', 'X', 'M', 'X'],
            vec!['M', 'M', 'A', 'M', 'S'],
            vec!['A', 'S', 'A', 'S', 'A'],
            vec!['X', 'A', 'M', 'X', 'S'],
        ];
        assert_eq!(is_xmas_third_quadrant(&matrix, 0, 1), false);
        assert_eq!(is_xmas_third_quadrant(&matrix, 1, 0), false);
        assert_eq!(is_xmas_third_quadrant(&matrix, 4, 0), false);
        assert_eq!(is_xmas_third_quadrant(&matrix, 4, 3), false);

        assert_eq!(is_xmas_third_quadrant(&matrix, 1, 2), false);

        assert_eq!(is_xmas_third_quadrant(&matrix, 0, 4), true);
        assert_eq!(is_xmas_third_quadrant(&matrix, 1, 4), false);
    }

    #[test]
    fn test_is_xmas_fourth_quadrant() {
        let matrix = vec![
            vec!['S', 'X', 'M', 'S', 'M'],
            vec!['X', 'S', 'A', 'M', 'X'],
            vec!['X', 'M', 'X', 'A', 'S'],
            vec!['M', 'M', 'A', 'M', 'X'],
            vec!['X', 'A', 'A', 'S', 'S'],
        ];
        assert_eq!(is_xmas_fourth_quadrant(&matrix, 4, 0), false);
        assert_eq!(is_xmas_fourth_quadrant(&matrix, 2, 0), false);

        assert_eq!(is_xmas_fourth_quadrant(&matrix, 1, 0), true);

        assert_eq!(is_xmas_fourth_quadrant(&matrix, 0, 1), false);
        assert_eq!(is_xmas_fourth_quadrant(&matrix, 1, 4), false);
    }
}
