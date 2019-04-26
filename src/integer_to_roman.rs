/// @number 12
/// @title Integer to Roman
/// @url https://leetcode.com/problems/integer-to-roman
/// @difficulty medium


struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num_clone = num;
        let mut ret = String::new();
        while num_clone > 0 {
            let x = match num_clone {
                1000...std::i32::MAX => ("M", 1000),
                900...1000 => ("CM", 900),
                500...900 => ("D", 500),
                400...500 => ("CD", 400),
                100...400 => ("C", 100),
                90...100 => ("XC", 90),
                50...90 => ("L", 50),
                40...50 => ("XL", 40),
                10...40 => ("X", 10),
                9 => ("IX", 9),
                5...9 => ("V", 5),
                4 => ("IV", 4),
                1...4 => ("I", 1),
                _ => unreachable!()
            };
            ret.push_str(x.0);
            num_clone -= x.1;
        }
        ret
    }
}


#[cfg(test)]
mod test {
    use crate::integer_to_roman::Solution;

    #[test]
    fn should_return_iii_given_3() {
        assert_eq!("III", &Solution::int_to_roman(3));
    }

    #[test]
    fn should_return_iv_given_4() {
        assert_eq!("IV", &Solution::int_to_roman(4));
    }

    #[test]
    fn should_return_ix_given_9() {
        assert_eq!("IX", &Solution::int_to_roman(9));
    }

    #[test]
    fn should_return_lviii_given_58() {
        assert_eq!("LVIII", &Solution::int_to_roman(58));
    }

    #[test]
    fn should_return_mcmxciv_given_1994() {
        assert_eq!("MCMXCIV", &Solution::int_to_roman(1994));
    }
}
