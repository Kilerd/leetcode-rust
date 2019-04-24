/// @number 852
/// @title Peak Index in a Mountain Array
/// @url https://leetcode.com/problems/peak-index-in-a-mountain-array
/// @difficulty easy


struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        for i in 1..=a.len() - 2 {
            if a[i - 1] < a[i] && a[i] > a[i + 1] {
                return i as i32;
            }
        }
        0
    }
}


#[test]
fn test1() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
}

#[test]
fn test2() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
}
