/// @number 8
/// @title String to Integer (atoi)
/// @url https://leetcode.com/problems/string-to-integer-atoi/
/// @difficulty Medium


struct Solution;

impl Solution {
    pub fn my_atoi(str_: String) -> i32 {
        use std::i32;
        let mut is_getting_number = false;
        let mut is_negative: Option<bool> = None;
        let mut ret = 0i32;
        let x = str_.trim().chars();
        for x in x {
            match x {
                ref n if n.is_ascii_digit() => {
                    let number_in_position = n.to_digit(10).unwrap() as i32;
                    let optional_ret: Option<i32> = ret.checked_mul(10).and_then(|inner| inner.checked_add(number_in_position));
                    if let Some(num) = optional_ret {
                        is_getting_number = true;
                        ret = num;
                    } else {
                        if let Some(true) = is_negative {
                            return std::i32::MIN;
                        }else {
                            return std::i32::MAX;
                        }
                    }
                }
                '+' => {
                    if is_negative.is_none() && is_getting_number == false {
                        is_negative = Some(false);
                    } else {
                        break;
                    }
                }
                '-' => {
                    if is_negative.is_none() && is_getting_number == false {
                        is_negative = Some(true);
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        };
        if let Some(true) = is_negative { -ret }else { ret }

    }
}


#[cfg(test)]
mod test {
    use crate::string_to_integer_atoi::Solution;

    #[test]
    fn should_work_with_normal_number() {
        assert_eq!(42i32, Solution::my_atoi("42".into()));
    }

    #[test]
    fn should_work_with_minus_mark() {
        assert_eq!(-42i32, Solution::my_atoi("-42".into()));
    }

    #[test]
    fn should_work_with_prefix_space() {
        assert_eq!(-42i32, Solution::my_atoi("   -42".into()));
    }

    #[test]
    fn should_work_with_postfix_external_words() {
        assert_eq!(4293i32, Solution::my_atoi("4293 with words".into()));
    }

    #[test]
    fn should_get_zero_with_prefix_words() {
        assert_eq!(0i32, Solution::my_atoi("with words  4293".into()));
    }

    #[test]
    fn should_check_overflow() {
        assert_eq!(std::i32::MIN, Solution::my_atoi("-91283472332".into()));
        assert_eq!(std::i32::MAX, Solution::my_atoi("91283472332".into()));
    }

    #[test]
    fn should_work_with_add_mark() {
        assert_eq!(42i32, Solution::my_atoi("+42".into()));
    }

    #[test]
    fn should_work_with_both_add_minus_mark() {
        assert_eq!(0i32, Solution::my_atoi("+-42".into()));
    }

    #[test]
    fn should_get_zero_with_internal_space() {
        assert_eq!(4i32, Solution::my_atoi("+4 2".into()));
    }

    #[test]
    fn test() {
        assert_eq!(4i32, Solution::my_atoi("+4 2".into()));
    }
}
