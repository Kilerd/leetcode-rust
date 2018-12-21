struct Solution();

impl Solution {
    pub fn to_lower_case(str: String) -> String {
        str.as_str().to_lowercase()
    }
}

fn main() {
    println!("Hello, world!");
}


#[test]
fn test() {
    assert_eq!(Solution::to_lower_case(String::from("HELLO")), String::from("hello"));
}