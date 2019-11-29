/// @number 33
/// @title Search in Rotated Sorted Array
/// @url https://leetcode.com/problems/search-in-rotated-sorted-array/
/// @difficulty Medium

struct Solution;

impl Solution {
    pub fn rotated_index(nums: &Vec<i32>, start: usize, end: usize) -> usize {
        if nums[start] < nums[end] {
            start
        } else {
            let mid = (end + start) / 2;
            if mid == start {
                return mid;
            }
            if nums[mid] < nums[start] {
                Solution::rotated_index(nums, start, mid)
            } else {
                Solution::rotated_index(nums, mid, end)
            }
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }

        let rotated_index = Solution::rotated_index(&nums, 0, nums.len() - 1);

        let index = if nums[0] <= target && target <= nums[rotated_index] {
            nums[0..=rotated_index].binary_search(&target)
        } else if rotated_index < nums.len() - 1 {
            if nums[rotated_index + 1] <= target && target <= nums[nums.len() - 1] {
                nums[rotated_index..nums.len()]
                    .binary_search(&target)
                    .map(|i| i + rotated_index)
            } else {
                Err(0)
            }
        } else {
            Err(0)
        };

        if let Ok(index) = index {
            index as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use crate::search_in_rotated_sorted_array::Solution;

    #[test]
    fn test_rotated() {
        assert_eq!(3, Solution::rotated_index(&vec![4, 5, 6, 7, 0, 1, 2], 0, 6))
    }

    #[test]
    fn test() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    }

    #[test]
    fn test2() {
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    }

    #[test]
    fn test3() {
        assert_eq!(7, Solution::search(vec![4, 5, 6, 7, 0, 1, 2, 3], 3));
    }
    #[test]
    fn test4() {
        assert_eq!(-1, Solution::search(vec![], 3));
    }

    #[test]
    fn test5() {
        assert_eq!(-1, Solution::search(vec![1], 3));
    }
    #[test]
    fn test6() {
        assert_eq!(0, Solution::search(vec![1], 1));
    }
}
