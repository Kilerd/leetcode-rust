/// @number 27
/// @title Remove Element
/// @url https://leetcode.com/problems/remove-element/
/// @difficulty easy


struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index = 0;
        while index < nums.len() {
            if nums[index] == val {
                nums.remove(index);
            }else{
                index += 1;
            }
        }
        nums.len() as i32
    }
}


#[cfg(test)]
mod test {
    use crate::remove_element::Solution;

    #[test]
    fn test1() {
        assert_eq!(2, Solution::remove_element(&mut vec![3,2,2,3], 3));
    }

    #[test]
    fn test2() {
        assert_eq!(5, Solution::remove_element(&mut vec![0,1,2,2,3,0,4,2], 2));
    }
}
