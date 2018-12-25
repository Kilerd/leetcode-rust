struct Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut odd = vec![];
        let mut even = vec![];
        let mut ret = vec![];
        a.into_iter().for_each(|e| {
            if e % 2 == 0 {
                even.push(e);
            } else {
                odd.push(e);
            }
        });
        for i in 0..odd.len() {
            ret.push(even[i]);
            ret.push(odd[i]);
        }
        ret
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test1() {
    assert_eq!(
        Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]),
        vec![4, 5, 2, 7]
    );
}
