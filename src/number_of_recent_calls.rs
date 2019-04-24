/// @number 933
/// @title Number of Recent Calls
/// @url https://leetcode.com/problems/number-of-recent-calls
/// @difficulty easy

struct RecentCounter {
    val: Vec<i32>,
}

/** You can modify the type of `self` for your need. */
impl RecentCounter {
    fn new() -> Self {
        Self { val: vec![] }
    }

    fn ping(&mut self, t: i32) -> i32 {
        use std::cmp::max;
        self.val.push(t);
        self.val = self
            .val
            .clone()
            .into_iter()
            .filter(|e| e >= max(&0, &(t - 3000)))
            .collect();
        self.val.len() as i32
    }
}
