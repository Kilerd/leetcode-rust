/// @number 6
/// @title ZigZag Conversion
/// @url https://leetcode.com/problems/zigzag-conversion
/// @difficulty medium

struct Solution;


impl Solution {

    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let chars: Vec<char> = s.chars().collect();

        let mut strings: Vec<String> = (0..num_rows).map(|_| String::new()).collect();
        let mut go_down = false;
        let mut loop_line = 0 as i32;

        for x in chars {
            strings[loop_line as usize].push(x);
            if loop_line == 0  || loop_line == num_rows - 1 {
                go_down = !go_down;
            }
            loop_line += if go_down { 1 } else { -1 };
        }
        strings.join("")
    }

}

#[cfg(test)]
mod test {
    use crate::zigzag_conversion::Solution;

    #[test]
    fn show_return_itself_when_number_is_1() {
        assert_eq!(String::from("PAYPALISHIRING"), Solution::convert(String::from("PAYPALISHIRING"), 1));

    }
    #[test]
    fn should_it_works() {
        assert_eq!(String::from("PAHNAPLSIIGYIR"), Solution::convert(String::from("PAYPALISHIRING"), 3));
    }
    #[test]
    fn should_it_works_too() {
        assert_eq!(String::from("PINALSIGYAHRPI"), Solution::convert(String::from("PAYPALISHIRING"), 4));
    }
}