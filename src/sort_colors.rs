/// @number 75
/// @title Sort Colors
/// @url https://leetcode.com/problems/sort-colors
/// @difficulty medium


struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let nums_len = nums.len().clone();
        let mut counts = [0; 3];

        for x in nums.as_slice() {
            counts[*x as usize] += 1;
        }

        for i in 0..nums_len {
            let value = if 0 <= i && i < counts[0] {
                0
            } else if counts[0] <= i && i < counts[0] + counts[1] {
                1
            } else {
                2
            };
            nums[i] = value;
        };
    }
}


#[cfg(test)]
mod test {
    use crate::sort_colors::Solution;

    #[test]
    fn should_sort_given_non_sorted_vec() {
        let mut vec1 = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut vec1);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], vec1);
    }
}