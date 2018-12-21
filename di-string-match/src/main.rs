struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut end = s.len() as i32;
        let mut start = 0;
        let mut ret : Vec<i32>= vec![];
        s.chars().for_each(|element| {
            match element {
                'I' => {
                    ret.push(start);
                    start += 1;
                },
                'D' => {
                    ret.push(end);
                    end -= 1;
                },
                _ => {}
            };
        });
        ret.push(start);
        ret
    }
}

fn main() {
    println!("Hello, world!");
}


#[test]
fn idid() {
    assert_eq!(Solution::di_string_match(String::from("IDID")), vec![0,4,1,3,2]);
}

#[test]
fn iii() {
    assert_eq!(Solution::di_string_match(String::from("III")), vec![0,1,2,3]);
}

#[test]
fn ddi() {
    assert_eq!(Solution::di_string_match(String::from("DDI")), vec![3,2,0,1]);
}