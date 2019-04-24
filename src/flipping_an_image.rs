/// @number 832
/// @title Flipping an Image
/// @url https://leetcode.com/problems/flipping-an-image
/// @difficulty easy

struct Solution;

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        a.into_iter()
            .map(|mut v| {
                v.reverse();
                v.into_iter()
                    .map(|element| element ^ 1)
                    .collect::<Vec<i32>>()
            })
            .collect()
    }
}


#[test]
fn test1() {
    assert_eq!(
        Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
        vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
    )
}

#[test]
fn test2() {
    assert_eq!(
        Solution::flip_and_invert_image(vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0]
        ]),
        vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 0, 1, 0]
        ]
    )
}
