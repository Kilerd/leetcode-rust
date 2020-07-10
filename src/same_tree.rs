/// @number 100
/// @title Same Tree
/// @url https://leetcode.com/problems/same-tree
/// @difficulty easy

//Given two binary trees, write a function to check if they are the same or not.
// 
//
// Two binary trees are considered the same if they are structurally identical a
//nd the nodes have the same value. 
//
// Example 1: 
//
// 
//Input:     1         1
//          / \       / \
//         2   3     2   3
//
//        [1,2,3],   [1,2,3]
//
//Output: true
// 
//
// Example 2: 
//
// 
//Input:     1         1
//          /           \
//         2             2
//
//        [1,2],     [1,null,2]
//
//Output: false
// 
//
// Example 3: 
//
// 
//Input:     1         1
//          / \       / \
//         2   1     1   2
//
//        [1,2,1],   [1,1,2]
//
//Output: false
// 
// Related Topics Tree Depth-first Search 
// 👍 2062 👎 59


struct Solution;
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
      right: None
    }
  }
}
//leetcode submit region begin(Prohibit modification and deletion)
// Definition for a binary tree node.

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(p_node), Some(q_node)) => {
                let (t1, t2) = (p_node.borrow(), q_node.borrow());
                let value_equals = t1.val == t2.val;

                let left_equals = Solution::is_same_tree(t1.left.clone(), t2.left.clone());
                let right_equals = Solution::is_same_tree(t1.right.clone(), t2.right.clone());
                value_equals && left_equals && right_equals
            },
            (None, None) => true,
            _ => false
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)




#[cfg(test)]
mod test {
    use crate::same_tree::{Solution, TreeNode};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test() {
        let mut node = TreeNode::new(1);
        node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        let mut node2 = TreeNode::new(1);
        node2.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        node2.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        assert_eq!(true, Solution::is_same_tree(
            Some(Rc::new(RefCell::new(node))),
            Some(Rc::new(RefCell::new(node2))),
        ));
    }

    #[test]
    fn test2() {
        let mut node = TreeNode::new(1);
        node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        let mut node2 = TreeNode::new(1);
        node2.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        assert_eq!(false, Solution::is_same_tree(
            Some(Rc::new(RefCell::new(node))),
            Some(Rc::new(RefCell::new(node2))),
        ));
    }
    #[test]
    fn test3() {
        let mut node = TreeNode::new(1);
        node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        node.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        let mut node2 = TreeNode::new(1);
        node2.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        node2.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        assert_eq!(false, Solution::is_same_tree(
            Some(Rc::new(RefCell::new(node))),
            Some(Rc::new(RefCell::new(node2))),
        ));
    }
}
