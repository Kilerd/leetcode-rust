struct Solution();

impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut even = Vec::new();
        let mut odd = Vec::new();
        a.into_iter().for_each(|element| {
            if element%2 == 0 {
                even.push(element);
            }else {
                odd.push(element);
            }
        });
        even.extend(odd);
        even

    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    assert_eq!(
        Solution::sort_array_by_parity(vec![2, 1, 4, 3]),
        vec![2, 4, 1, 3]
    );
}
