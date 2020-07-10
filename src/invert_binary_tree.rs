/// @number 226
/// @title Invert Binary Tree
/// @url https://leetcode.com/problems/invert-binary-tree
/// @difficulty easy

//Invert a binary tree. 
//
// Example: 
//
// Input: 
//
// 
//     4
//   /   \
//  2     7
// / \   / \
//1   3 6   9 
//
// Output: 
//
// 
//     4
//   /   \
//  7     2
// / \   / \
//9   6 3   1 
//
// Trivia: 
//This problem was inspired by this original tweet by Max Howell: 
//
// Google: 90% of our engineers use the software you wrote (Homebrew), but you c
//an’t invert a binary tree on a whiteboard so f*** off. 
// Related Topics Tree 
// 👍 3357 👎 55


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
            right: None,
        }
    }
}
//leetcode submit region begin(Prohibit modification and deletion)
// Definition for a binary tree node.

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_node) = root {
            {
                let mut ref_mut = root_node.borrow_mut();
                let inverted_left_child = Solution::invert_tree(ref_mut.left.take());
                let inverted_right_child = Solution::invert_tree(ref_mut.right.take());

                ref_mut.left = inverted_right_child;
                ref_mut.right = inverted_left_child;
            }
            Some(root_node)
        } else {
            None
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

