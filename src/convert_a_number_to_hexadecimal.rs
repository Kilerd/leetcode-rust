use std::ops::{Shl, ShlAssign, Shr};
use std::hint::unreachable_unchecked;

/// @number 405
/// @title Convert a Number to Hexadecimal
/// @url https://leetcode.com/problems/convert-a-number-to-hexadecimal
/// @difficulty easy

//
//Given an integer, write an algorithm to convert it to hexadecimal. For negativ
//e integer, two’s complement method is used.
// 
//
// Note:
// 
// All letters in hexadecimal (a-f) must be in lowercase. 
// The hexadecimal string must not contain extra leading 0s. If the number is ze
//ro, it is represented by a single zero character '0'; otherwise, the first chara
//cter in the hexadecimal string will not be the zero character. 
// The given number is guaranteed to fit within the range of a 32-bit signed int
//eger. 
// You must not use any method provided by the library which converts/formats th
//e number to hex directly. 
// 
// 
//
// Example 1:
// 
//Input:
//26
//
//Output:
//"1a"
// 
// 
//
// Example 2:
// 
//Input:
//-1
//
//Output:
//"ffffffff"
// 
// Related Topics Bit Manipulation 
// 👍 433 👎 112

struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 { return "0".into(); }


        let mut leading_zero = false;
        let mut ret = String::new();
        let mask: u32 = 0xf0000000;

        let mut looping_num: u32 = num as u32;
        for i in 0..8 {
            let i1 = (looping_num & mask) >> 28;
            let position_char = match i1 {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                10 => 'a',
                11 => 'b',
                12 => 'c',
                13 => 'd',
                14 => 'e',
                15 => 'f',
                _ => unreachable!()
            };

            if position_char != '0' || leading_zero == true {
                ret.push(position_char);
                leading_zero = true;
            }

            looping_num <<= 4;
        }
        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod test {
    use crate::convert_a_number_to_hexadecimal::Solution;

    #[test]
    fn test1() {
        assert_eq!("1a".to_string(), Solution::to_hex(26));
    }

    #[test]
    fn test2() {
        assert_eq!("0", Solution::to_hex(0));
    }

    #[test]
    fn test3() {
        assert_eq!("ffffffff", Solution::to_hex(-1))
    }
}
