struct Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut ret = vec![];
        for index in left..=right {
            let mut index_copy = index;
            while index_copy > 0 {
                let bit = index_copy % 10;
                if bit == 0 || index % bit !=0 {
                    break
                }
                index_copy /=10;
            };
            if index_copy == 0 {
                ret.push(index);
            }
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
        Solution::self_dividing_numbers(1, 22),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
    );
}
