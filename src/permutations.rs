/// @number 46
/// @title Permutations
/// @url https://leetcode.com/problems/permutations
/// @difficulty medium

//Given a collection of distinct integers, return all possible permutations. 
//
// Example: 
//
// 
//Input: [1,2,3]
//Output:
//[
//  [1,2,3],
//  [1,3,2],
//  [2,1,3],
//  [2,3,1],
//  [3,1,2],
//  [3,2,1]
//]
// 
// Related Topics Backtracking 
// 👍 3843 👎 106


//leetcode submit region begin(Prohibit modification and deletion)
struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 1 { return vec![nums]; };

        let mut ret = vec![];
        for i in 0..nums.len() {
            let mut iter_ret = vec![];
            let mut cloned_nums = nums.clone();
            let index_item = cloned_nums.remove(i);
            iter_ret.push(index_item);
            for x in Solution::permute(cloned_nums) {
                let mut cloned_iter_ret = iter_ret.clone();
                cloned_iter_ret.extend(x);
                ret.push(cloned_iter_ret);
            }
        }
        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod test {
    use crate::permutations::Solution;

    #[test]
    fn test() {
        let ret = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ];
        assert_eq!(ret, Solution::permute(vec![1, 2, 3]));
    }
}
