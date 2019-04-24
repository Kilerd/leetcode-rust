/// @number 1
/// @title Two Sum
/// @url https://leetcode.com/problems/two-sum
/// @difficulty easy


struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut r: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            r.insert(nums[i], i as i32);
        }

        for i in 0..nums.len() {
            match r.get(&(target - nums[i])) {
                Some(index) if i as i32 != *index => return vec![i as i32, index.clone()],
                None => {}
                _ => {}
            }
        }
        vec![]
    }
}


#[test]
fn test1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn test2() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9999), vec![]);
}

#[test]
fn test3() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}
