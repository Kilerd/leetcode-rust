struct Solution();

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        match(n) {
            1...3 => true,
            _ if n%4 == 0 => false,
            _ => true
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    assert!(!Solution::can_win_nim(4));
    assert!(Solution::can_win_nim(3));
    assert!(Solution::can_win_nim(6));
}