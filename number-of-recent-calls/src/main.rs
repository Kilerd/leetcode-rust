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

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

fn main() {
    println!("Hello, world!");
}
