/// @number 9
/// @title Palindrome Number
/// @url https://leetcode.com/problems/palindrome-number/
/// @difficulty easy


struct Solution;


impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; }

        let string = x.to_string();
        let mut chars: Vec<char> = string.chars().collect();
        while chars.len() > 1 {
            let last = chars.pop().unwrap();
            let first = chars.remove(0);
            if first != last {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::palindrome_number::Solution;

    #[test]
    fn should_return_true_given_121() {
        assert_eq!(true, Solution::is_palindrome(121));
    }
    #[test]
    fn should_return_false_given_negative_number() {
        assert_eq!(false, Solution::is_palindrome(-121));
    }
    #[test]
    fn should_return_true_given_zero() {
        assert_eq!(true, Solution::is_palindrome(0));
    }

    #[test]
    fn should_return_false_given_10() {
        assert_eq!(false, Solution::is_palindrome(10));
    }
}
