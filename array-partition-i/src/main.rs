
struct Solution;

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut numss = nums.clone();
        let mut sum = 0;
        numss.sort();
        for i in (0..numss.len()).step_by(2) {
            sum+= numss[i];
        }
        sum
    }
}

fn main() {
    println!("Hello, world!");
}


#[test]
fn test1() {
    assert_eq!(Solution::array_pair_sum(vec![1,4,3,2]), 4);
}