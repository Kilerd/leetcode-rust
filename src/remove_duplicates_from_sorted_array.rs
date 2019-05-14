/// @number 16
/// @title Remove Duplicates from Sorted Array
/// @url https://leetcode.com/problems/remove-duplicates-from-sorted-array/
/// @difficulty easy


struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use crate::remove_duplicates_from_sorted_array::Solution;

    #[test]
    fn should_return_2_given_1_1_2() {
        assert_eq!(2, Solution::remove_duplicates(&mut vec![1, 1, 2]));
    }

    #[test]
    fn should_return_5_given_0_0_1_1_1_2_2_3_3_4() {
        assert_eq!(5, Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]));
    }
}
