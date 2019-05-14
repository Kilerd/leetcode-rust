/// @number 17
/// @title Letter Combinations of a Phone Number
/// @url https://leetcode.com/problems/letter-combinations-of-a-phone-number/
/// @difficulty medium


struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let len = digits.len();
        if len < 1 { return vec![]; };

        let x = &digits[len - 1..];
        let letter_chars = match x {
            "2" => vec!["a", "b", "c"],
            "3" => vec!["d", "e", "f"],
            "4" => vec!["g", "h", "i"],
            "5" => vec!["j", "k", "l"],
            "6" => vec!["m", "n", "o"],
            "7" => vec!["p", "q", "r", "s"],
            "8" => vec!["t", "u", "v"],
            "9" => vec!["w", "x", "y", "z"],
            _ => vec![]
        };
        let letter_chars_len = letter_chars.len();

        if len == 1 {
            return letter_chars.iter().map(|c| String::from(*c)).collect::<Vec<String>>();
        }

        if letter_chars_len < 1 {
            return Solution::letter_combinations(String::from(&digits[..len - 1]));
        }


        let prefix = Solution::letter_combinations(String::from(&digits[..len - 1]));

        let mut ret = vec![];
        if prefix.len() > 0 {
            for x in prefix {
                for y in letter_chars.clone() {
                    ret.push(format!("{}{}", x.clone(), y.clone()));
                }
            }
            ret
        } else {
            letter_chars.iter().map(|c| String::from(*c)).collect::<Vec<String>>()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::letter_combinations_of_a_phone_number::Solution;
    use std::vec::Vec;

    #[test]
    fn should_work_with_one_letter() {
        assert_eq!(
            vec!["a", "b", "c"].iter().map(|c| String::from(*c)).collect::<Vec<String>>(),
            Solution::letter_combinations("2".into())
        );
    }

    #[test]
    fn should_work_with_two_letter() {
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].iter().map(|c| String::from(*c)).collect::<Vec<String>>(),
            Solution::letter_combinations("23".into())
        );
    }

    #[test]
    fn should_work_with_two_letter_with_invalid_letter() {
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].iter().map(|c| String::from(*c)).collect::<Vec<String>>(),
            Solution::letter_combinations("213".into())
        );
    }
}
