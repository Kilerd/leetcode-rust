/// @number 804
/// @title Unique Morse Code Words
/// @url https://leetcode.com/problems/unique-morse-code-words
/// @difficulty easy


struct Solution();

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];

        let mut words_morse = words
            .into_iter()
            .map(|word| {
                word.chars()
                    .map(|each_char| {
                        let index = each_char as usize - 97;
                        morse[index].to_string()
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>();

        words_morse.sort();
        words_morse.dedup_by(|a, b| a == b);

        words_morse.len() as i32
    }
}


#[test]
fn test1() {
    assert_eq!(
        Solution::unique_morse_representations(vec![
            String::from("gin"),
            String::from("zen"),
            String::from("gig"),
            String::from("msg"),
        ]),
        2
    );
}
