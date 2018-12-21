struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut ret = x ^ y;
        let mut sum = 0;
        while ret != 0 {
            sum += ret % 2;
            ret /= 2;
        };
        sum
    }
}

fn main() {
    println!("Hello, world!");
}


#[test]
fn test() {
    assert_eq!(Solution::hamming_distance(1, 4), 2);
    assert_eq!(Solution::hamming_distance(1, 0), 1);
}