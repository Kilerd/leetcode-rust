/// @number 28
/// @title Implement strStr()
/// @url https://leetcode.com/problems/implement-strstr/
/// @difficulty easy


struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_len = haystack.len();
        let needle_len = needle.len();

        if needle_len == 0 { return 0; }

        let mut i = 0;
        'main: while (i as i32) <= (haystack_len as i32 - needle_len as i32) {
            let mut j = 0;
            if !&haystack[i..i + 1].eq(&needle[j..j + 1]) {
                i += 1;
                continue;
            }
            let mut k = i;
            while j < needle_len {
                if !&haystack[k..k + 1].eq(&needle[j..j + 1]) {
                    i += 1;
                    continue 'main;
                }
                k += 1;
                j += 1;
            }
            return i as i32;
        }
        -1
    }
}


#[cfg(test)]
mod test {
    use crate::implement_strstr::Solution;

    #[test]
    fn test1() {
        assert_eq!(2, Solution::str_str("hello".into(), "ll".into()));
    }

    #[test]
    fn test2() {
        assert_eq!(-1, Solution::str_str("aaaaa".into(), "-1".into()));
    }

    #[test]
    fn test3() {
        assert_eq!(0, Solution::str_str("aaaaa".into(), "".into()));
    }

    #[test]
    fn test4() {
        assert_eq!(-1, Solution::str_str("".into(), "a".into()));
    }

    #[test]
    fn test5() {
        assert_eq!(-1, Solution::str_str("aaaa".into(), "aaaaa".into()));
    }

    #[test]
    fn test6() {
        assert_eq!(0, Solution::str_str("a".into(), "a".into()));
    }

    #[test]
    fn test7() {
        assert_eq!(4, Solution::str_str("mississippi".into(), "issip".into()));
    }
}
