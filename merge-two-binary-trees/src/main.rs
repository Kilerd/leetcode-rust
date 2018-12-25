// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (t1, t2) {
            (t, None) | (None, t) => t,
            (Some(t1), Some(t2)) => {
                let (t1, t2) = (t1.borrow(), t2.borrow());
                Some(Rc::new(RefCell::new(TreeNode {
                    val: t1.val + t2.val,
                    left: Solution::merge_trees(t1.left.clone(), t2.left.clone()),
                    right: Solution::merge_trees(t1.right.clone(), t2.right.clone()),
                })))
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    unimplemented!();
}
