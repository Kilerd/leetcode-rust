/// @number 7
/// @title Reverse Integer
/// @url https://leetcode.com/problems/reverse-integer/
/// @difficulty easy


struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        use std::i32;
        let mut ret = 0i32;
        let mut x_clone = x;
        while x_clone != 0 {
            let i = x_clone % 10;
            let x1 = ret.overflowing_mul(10);
            if x1.1 == true {
                return 0;
            }
            let x2 = x1.0.overflowing_add(i);
            if x2.1 == true {
                return 0;
            }
            ret = x2.0;
            x_clone = x_clone / 10;
        }

        ret
    }
}


#[cfg(test)]
mod test {
    use crate::reverse_integer::Solution;

    #[test]
    fn should_return_321_given_123() {
        assert_eq!(321, Solution::reverse(123));
    }

    #[test]
    fn should_return_negative_321_given_negative_123() {
        assert_eq!(-321, Solution::reverse(-123));
    }

    #[test]
    fn should_return_21_given_120() {
        assert_eq!(21, Solution::reverse(120));
    }

    #[test]
    fn should_return_0_given_1534236469() {
        assert_eq!(0, Solution::reverse(1534236469));
    }
}
