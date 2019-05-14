/// @number 20
/// @title Valid Parentheses
/// @url https://leetcode.com/problems/valid-parentheses/
/// @difficulty easy


struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let chars: Vec<char> = s.chars().collect();
        for x in chars {
            match x {
                '(' => stack.push('('),
                '{' => stack.push('{'),
                '[' => stack.push('['),
                ')' => {
                    match stack.pop() {
                        Some('(') => continue,
                        _ => return false
                    }
                },
                '}' => {
                    match stack.pop() {
                        Some('{') => continue,
                        _ => return false
                    }
                },
                ']' => {
                    match stack.pop() {
                        Some('[') => continue,
                        _ => return false
                    }
                },
                _=> return false
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use crate::valid_parentheses::Solution;

    #[test]
    fn test1() {
        assert_eq!(true, Solution::is_valid("()".into()));
    }
    #[test]
    fn test2() {
        assert_eq!(true, Solution::is_valid("()[]{}".into()));
    }
    #[test]
    fn test3() {
        assert_eq!(false, Solution::is_valid("(]".into()));
    }
    #[test]
    fn test4() {
        assert_eq!(false, Solution::is_valid("([)]".into()));
    }
    #[test]
    fn test5() {
        assert_eq!(true, Solution::is_valid("{[]}".into()));
    }
}
