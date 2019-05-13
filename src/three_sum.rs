/// @number 15
/// @title 3Sum
/// @url https://leetcode.com/problems/3sum/
/// @difficulty medium


struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::{HashMap, HashSet};
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut set = HashSet::new();
        let len = nums.len();

        for x in nums.clone() {
            let value = map.get(&x).unwrap_or(&0);
            map.insert(x, *value + 1);
        }

        for i in 0..len {
            for j in i + 1..len {
                let i1 = 0 - nums[i] - nums[j];
                let except_size = if i1 == nums[i] && i1 == nums[j] {
                    2
                } else if i1 == nums[i] || i1 == nums[j] {
                    1
                } else {
                    0
                };
                if map.get(&i1).unwrap_or(&0) > &except_size {
                    let mut vec1 = vec![nums[i], nums[j], i1];
                    vec1.sort();
                    set.insert(vec1);
                }
            }
        }
        set.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::three_sum::Solution;
    use std::collections::HashSet;

    fn assert_vec(v1:Vec<Vec<i32>>, v2:Vec<Vec<i32>>) {
        let mut set1 = HashSet::new();
        for x in v1 {
            set1.insert(x);
        }
        let mut set2 = HashSet::new();
        for x in v2 {
            set2.insert(x);
        }

        assert_eq!(set1, set2);

    }

    #[test]
    fn test() {
        assert_vec(
            vec![
                vec![-1, 0, 1],
                vec![-1, -1, 2]
            ],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
    }

    #[test]
    fn test2() {
        assert_vec(
            vec![
                vec![-1, 0, 1],
            ],
            Solution::three_sum(vec![-1, 0, 1, 0])
        );
    }
}
