/// @number 36
/// @title Valid Sudoku
/// @url https://leetcode.com/problems/valid-sudoku/
/// @difficulty Medium

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        //line validate
        for line in &board {
            let mut set = HashSet::new();
            for c in line {
                if c != &'.' {
                    if set.contains(c) {
                        return false;
                    }
                    set.insert(c);
                }
            }
        }

        // column validate
        for i in 0..9 {
            let mut set = HashSet::new();
            for j in 0..9 {
                let x = &board[j][i];
                if x != &'.' {
                    if set.contains(x) {
                        return false;
                    }
                    set.insert(x);
                }
            }
        }

        //sub-box validate
        for i in 0..3 {
            let i = i * 3;
            for j in 0..3 {
                let j = j * 3;

                let mut set = HashSet::new();

                for k in 0..3 {
                    for l in 0..3 {
                        let x = &board[i + k][j + l];
                        if x != &'.' {
                            if set.contains(x) {
                                return false;
                            }
                            set.insert(x);
                        }
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use crate::valid_sudoku::Solution;

    #[test]
    fn test() {
        let sudoku = vec![
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

        assert_eq!(true, Solution::is_valid_sudoku(sudoku));
    }

    #[test]
    fn test2() {
        let sudoku = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(false, Solution::is_valid_sudoku(sudoku));
    }
}
