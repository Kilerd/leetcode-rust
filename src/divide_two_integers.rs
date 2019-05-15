/// @number 29
/// @title Divide Two Integers
/// @url https://leetcode.com/problems/divide-two-integers/
/// @difficulty medium

struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == std::i32::MIN && divisor == -1 {
            return std::i32::MAX;
        }
        let minus = dividend >= 0 && divisor < 0 || dividend < 0 && divisor >= 0;

        let mut dividend_ = (dividend as i64).abs();
        let divisor_ = (divisor as i64).abs();

        let mut quotient: i64 = 0;
        let mut shift = 0;
        let mut temp = divisor_;
        while dividend_ >= divisor_ {
            if dividend_ >= temp {
                dividend_ -= temp;
                quotient += 1 << shift;
                shift += 1;
                temp <<= 1;
            } else {
                shift -= 1;
                temp >>= 1;
            }
        }

        if minus {
            -quotient as i32
        } else {
            quotient as i32
        }
    }
}

#[cfg(test)]
mod test {
    use crate::divide_two_integers::Solution;

    #[test]
    fn test1() {
        assert_eq!(3, Solution::divide(10, 3));
    }

    #[test]
    fn test2() {
        assert_eq!(-2, Solution::divide(7, -3));
    }

    #[test]
    fn test3() {
        assert_eq!(0, Solution::divide(0, -3));
    }

    #[test]
    fn test4() {
        assert_eq!(2, Solution::divide(-7, -3));
    }

    #[test]
    fn test5() {
        assert_eq!(-2, Solution::divide(-7, 3));
    }

    #[test]
    fn test6() {
        assert_eq!(2_147_483_647, Solution::divide(-2_147_483_648, -1));
    }

    #[test]
    fn test7() {
        assert_eq!(-2_147_483_648, Solution::divide(-2_147_483_648, 1));
    }
    #[test]
    fn test8() {
        assert_eq!(0, Solution::divide(-999_511_578, -2_147_483_648));
    }
    #[test]
    fn test9() {
        assert_eq!(75, Solution::divide(31_234_231, 413_123));
    }
}
