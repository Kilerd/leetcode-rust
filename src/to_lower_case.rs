/// @number 709
/// @title To Lower Case
/// @url https://leetcode.com/problems/to-lower-case
/// @difficulty easy


struct Solution();

impl Solution {
    pub fn to_lower_case(str: String) -> String {
        str.as_str().to_lowercase()
    }
}


#[test]
fn test() {
    assert_eq!(Solution::to_lower_case(String::from("HELLO")), String::from("hello"));
}