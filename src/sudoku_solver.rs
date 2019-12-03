/// @number 37
/// @title Sudoku Solver
/// @url https://leetcode.com/problems/sudoku-solver/
/// @difficulty Hard

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut empty_position = vec![];
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    empty_position.push((i, j));
                    board[i][j] = '0';
                }
            }
        }
        let position_len = empty_position.len();

        let mut loop_index = 0;
        while loop_index < position_len {
            let (x, y) = empty_position[loop_index];

            if board[x][y] < '9' {
                board[x][y] = std::char::from_u32((board[x][y] as u32) + 1).unwrap();
            } else {
                board[x][y] = '0';
                loop_index -= 1;
                continue;
            }
            if Solution::invalid(board, x, y) {
                continue;
            }
            loop_index += 1;
        }
    }
    pub fn invalid(board: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        use std::collections::HashSet;
        let mut contain_table = HashSet::new();

        // validate line
        for i in 0..9 {
            let x1 = board[x][i];
            if x1 != '0' {
                if contain_table.contains(&x1) {
                    return true;
                }
                contain_table.insert(x1);
            }
        }
        contain_table.clear();

        // validate column

        for i in 0..9 {
            let x2 = board[i][y];
            if x2 != '0' {
                if contain_table.contains(&x2) {
                    return true;
                }
                contain_table.insert(x2);
            }
        }

        contain_table.clear();
        // validate box

        let i1 = x / 3;
        let i2 = y / 3;
        for i in 0..3 {
            for j in 0..3 {
                let x3 = board[i1 * 3 + i][i2 * 3 + j];
                if x3 != '0' {
                    if contain_table.contains(&x3) {
                        return true;
                    }
                    contain_table.insert(x3);
                }
            }
        }
        contain_table.clear();
        false
    }
}

#[cfg(test)]
mod test {
    use crate::sudoku_solver::Solution;

    #[test]
    fn test() {
        let mut sudoku = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut sudoku);

        let mut expected_solution = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        assert_eq!(expected_solution, sudoku);
    }

    #[test]
    fn test_invalid_method() {
        let mut expected_solution = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        for i in 0..9 {
            for j in 0..9 {
                assert_eq!(false, Solution::invalid(&expected_solution, i, j));
            }
        }
    }
}
