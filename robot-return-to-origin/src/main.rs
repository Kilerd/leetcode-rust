struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut vertical = 0;
        let mut horizon = 0;
        moves.chars().for_each(|element| {
            match element {
                'U' => {horizon += 1;}
                'D' => {horizon -= 1;}
                'L' => {vertical += 1;}
                'R' => {vertical -= 1;}
                _ => {}
            };
        });

        vertical == 0 && horizon == 0
    }
}

fn main() {
    println!("Hello, world!");
}


#[test]
fn test() {
    assert!(Solution::judge_circle(String::from("UUDD")));
    assert!(!Solution::judge_circle(String::from("UUUUU")));
    assert!(Solution::judge_circle(String::from("RLUURDDDLU")));
}