struct Solution();

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut sum: i32 = 0;
        j.chars().for_each(|each_char| {
            s.chars().for_each(|inner_char|{
                if inner_char == each_char {
                    sum += 1;
                }
            });
        });
        sum
    }
}

fn main() {
    println!("Hello, world!");
}


#[test]
fn all_test() {
    assert_eq!(Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
    assert_eq!(Solution::num_jewels_in_stones("z".to_string(), "zZ".to_string()), 1);
}