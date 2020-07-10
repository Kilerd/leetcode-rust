/// @number 104
/// @title Maximum Depth of Binary Tree
/// @url https://leetcode.com/problems/maximum-depth-of-binary-tree
/// @difficulty easy

//Given a binary tree, find its maximum depth. 
//
// The maximum depth is the number of nodes along the longest path from the root
// node down to the farthest leaf node. 
//
// Note: A leaf is a node with no children. 
//
// Example: 
//
// Given binary tree [3,9,20,null,null,15,7], 
//
// 
//    3
//   / \
//  9  20
//    /  \
//   15   7 
//
// return its depth = 3. 
// Related Topics Tree Depth-first Search 
// 👍 2511 👎 73


use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

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

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // using reference to avoid clone
        pub fn _max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(root_node) = root {
                let node_borrow = root_node.borrow();
                let left_depth = _max_depth(&node_borrow.left);
                let right_depth = _max_depth(&node_borrow.right);
                std::cmp::max(left_depth, right_depth) + 1
            }else {
                0
            }
        }
        _max_depth(&root)
    }
}
//leetcode submit region end(Prohibit modification and deletion)




#[cfg(test)]
mod test {
    use crate::maximum_depth_of_binary_tree::{Solution, TreeNode};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test() {
        let node15 = TreeNode::new(15);
        let node7 = TreeNode::new(7);

        let mut node20 = TreeNode::new(20);
        node20.left = Some(Rc::new(RefCell::new(node15)));
        node20.right = Some(Rc::new(RefCell::new(node7)));

        let node9 = TreeNode::new(9);

        let mut node3 = TreeNode::new(3);
        node3.left = Some(Rc::new(RefCell::new(node20)));
        node3.right = Some(Rc::new(RefCell::new(node9)));

        assert_eq!(3, Solution::max_depth(Some(Rc::new(RefCell::new(node3)))));
    }

    #[test]
    fn test2() {
        assert_eq!(0, Solution::max_depth(None))
    }
}
