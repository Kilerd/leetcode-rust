
/// @number 39
/// @title Combination Sum
/// @url https://leetcode.com/problems/combination-sum/
/// @difficulty medium


struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        for x in &candidates {
            match target.cmp(x) {
                std::cmp::Ordering::Greater => {
                    let mut vec1 = Solution::combination_sum(candidates.clone(), target - *x);
                    for mut inner in vec1 {
                        inner.push(*x);
                        inner.sort();
                        ret.push(inner);
                    }
                }
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => {
                    ret.push(vec![target]);
                }
            }

        }
        ret.sort();
        ret.dedup_by(|a, b| {
            a == b
        });
        ret
    }
}



#[test]
fn all_test() {
    assert_eq!(Solution::combination_sum(vec![2,3,6,7], 7), vec![ vec![2,2,3],vec![7]]);
    assert_eq!(Solution::combination_sum(vec![2,3,5], 7), vec![ vec![2,2,3],vec![2,5]]);
}