use crate::list::ListNode;

/// @number 2
/// @title Add Two Numbers
/// @url https://leetcode.com/problems/add-two-numbers
/// @difficulty medium


struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut cursor = &mut head;
        let mut one = &l1;
        let mut two = &l2;

        let mut step_in = false;
        while one.is_some() || two.is_some() {
            let one_value = if one.is_some() { one.as_ref().unwrap().val } else { 0 };
            let two_value = if two.is_some() { two.as_ref().unwrap().val } else { 0 };
            let count = (one_value + two_value + if step_in { 1 } else { 0 }) % 10;
            step_in = (one_value + two_value + if step_in { 1 } else { 0 }) > 9;
            *cursor = Some(Box::new(ListNode { val: count, next: None }));
            cursor = &mut cursor.as_mut().unwrap().next;

            if one.is_some() { one = &one.as_ref().unwrap().next };
            if two.is_some() { two = &two.as_ref().unwrap().next };
        }

        if step_in {
            *cursor = Some(Box::new(ListNode{ val: 1, next: None }))
        }

        head
    }
}

#[cfg(test)]
mod test {
    use crate::add_two_number::Solution;
    use crate::list::ListNode;

    #[test]
    fn should_return_708() {
        let l1 = ListNode::build_from_vec(vec![2, 4, 3]);
        let l2 = ListNode::build_from_vec(vec![5, 6, 4]);

        let ret = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::build_from_vec(vec![7, 0, 8]), ret);
    }
    #[test]
    fn should_return_01() {
        let l1 = ListNode::build_from_vec(vec![5]);
        let l2 = ListNode::build_from_vec(vec![5]);

        let ret = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::build_from_vec(vec![0, 1]), ret);
    }
}