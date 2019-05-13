/// @number 11
/// @title Container With Most Water
/// @url https://leetcode.com/problems/container-with-most-water/
/// @difficulty medium


struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::{min, max};
//        let len = height.len();
//        let mut max_size = 0;
//        for start in 0..len {
//            for end in start..len {
//                let current_max = (end - start) as i32 * min(height[start], height[end]);
//                max_size = max(current_max, max_size);
//            }
//        }
//        max_size

        /// two pointer solution

        let mut start = 0usize;
        let mut end = height.len() - 1;
        let mut max_size = 0;
        while start < end {
            let current_max = (end - start) as i32 * min(height[start], height[end]);
            max_size = max(current_max, max_size);
            if height[start] < height[end] {
                start += 1;
            } else {
                end -= 1;
            }
        }

        max_size
    }
}


#[cfg(test)]
mod test {
    use crate::container_with_most_water::Solution;

    #[test]
    fn test() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }
}
