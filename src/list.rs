#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    pub fn build_from_vec(vec: Vec<i32>)-> Option<Box<ListNode>> {
        let mut res : Option<Box<ListNode>> = None;
        for v in vec.iter().rev() {
            res = Some(Box::new(ListNode {val: *v, next: res }));
        }
        res
    }
}
