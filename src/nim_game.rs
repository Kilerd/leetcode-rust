/// @number 292
/// @title Nim Game
/// @url https://leetcode.com/problems/nim-game
/// @difficulty easy

struct Solution();

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        match n {
            1...3 => true,
            _ if n % 4 == 0 => false,
            _ => true
        }
    }
}


#[test]
fn test() {
    assert!(!Solution::can_win_nim(4));
    assert!(Solution::can_win_nim(3));
    assert!(Solution::can_win_nim(6));
}