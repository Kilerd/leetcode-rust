

/// @number 16
/// @title 3Sum Closest
/// @url https://leetcode.com/problems/3sum-closest/
/// @difficulty medium


struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::min;
        let mut nums_: Vec<i32> = nums.clone();
        nums_.sort();
        let mut diff = nums_[0] + nums_[1] + nums_[2];
        let len = nums_.len();
        for i in 0..len - 2 {
            let mut start = i + 1;
            let mut end = len - 1;
            while start < end {
                let current_sum = nums_[i] + nums_[start] + nums_[end];
                if target == current_sum {
                    return target;
                }
                if current_sum < target {
                    start += 1;
                } else {
                    end -= 1;
                }

                if (diff - target).abs() > (current_sum - target).abs() {
                    diff = current_sum;
                }
            }
        }
        diff
    }
}

#[cfg(test)]
mod test {
    use crate::three_sum_closest::Solution;

    #[test]
    fn test() {
        assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    }
}
