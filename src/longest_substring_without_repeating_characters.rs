/// @number 3
/// @title Longest Substring Without Repeating Characters
/// @url https://leetcode.com/problems/longest-substring-without-repeating-characters/
/// @difficulty medium
/// TODO: Sliding Window Optimized


struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;
        use std::cmp::max;

        let chars: Vec<char> = s.chars().collect();
        let mut set: HashSet<char> = HashSet::new();
        let mut max_length = 0;
        for i in 0..chars.len() {
            for j in i..chars.len() {
                if !set.contains(&chars[j]) {
                    set.insert(chars[j]);
                } else {
                    max_length = max(max_length, set.len());
                    set.clear();
                    break;
                }
            }
        }
        max(max_length, set.len()) as i32
    }
}

#[cfg(test)]
mod test {
    use crate::longest_substring_without_repeating_characters::Solution;

    #[test]
    fn should_return_3_given_abcabcbb() {
        assert_eq!(3, Solution::length_of_longest_substring(String::from("abcabcbb")));
    }

    #[test]
    fn should_return_1_given_bbbbb() {
        assert_eq!(1, Solution::length_of_longest_substring(String::from("bbbbb")));
    }

    #[test]
    fn should_return_3_given_pwwkew() {
        assert_eq!(3, Solution::length_of_longest_substring(String::from("pwwkew")));
    }

    #[test]
    fn should_return_0_given_empty() {
        assert_eq!(1, Solution::length_of_longest_substring(String::from(" ")));
    }

    #[test]
    fn should_return_0_given_empty_string() {
        assert_eq!(0, Solution::length_of_longest_substring(String::from("")));
    }
}