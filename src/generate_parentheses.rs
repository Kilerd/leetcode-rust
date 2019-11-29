/// @number 22
/// @title Generate Parentheses
/// @url https://leetcode.com/problems/generate-parentheses/
/// @difficulty Medium

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 0 {
            return vec!["".to_string()];
        }
        let mut ans = vec![];
        for i in 0..=n - 1 {
            for left in Solution::generate_parenthesis(n - 1 - i) {
                for right in Solution::generate_parenthesis(i) {
                    ans.push(format!("({}){}", left, right));
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::generate_parentheses::Solution;

    #[test]
    fn test() {
        let expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        let expected: Vec<String> = expected.into_iter().map(|item| item.to_string()).collect();
        assert_eq!(expected, Solution::generate_parenthesis(3));
    }
    #[test]
    fn test_n_equals_0() {
        let expected: Vec<String> = vec!["".to_string()];
        assert_eq!(expected, Solution::generate_parenthesis(0));
    }
}
