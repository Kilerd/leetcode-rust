/// @number 102
/// @title Binary Tree Level Order Traversal
/// @url https://leetcode.com/problems/binary-tree-level-order-traversal
/// @difficulty medium

//Given a binary tree, return the level order traversal of its nodes' values. (i
//e, from left to right, level by level). 
//
// 
//For example: 
//Given binary tree [3,9,20,null,null,15,7], 
// 
//    3
//   / \
//  9  20
//    /  \
//   15   7
// 
// 
// 
//return its level order traversal as: 
// 
//[
//  [3],
//  [9,20],
//  [15,7]
//]
// 
// Related Topics Tree Breadth-first Search 
// 👍 2994 👎 74


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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut vec1 = vec![root];
        let mut vec2 = vec![];

        let mut looper_flag = true;

        let mut ret = vec![];

        loop {
            if vec1.is_empty() && vec2.is_empty() {
                break;
            }
            let (looper, child_inserter) = if looper_flag {
                (&mut vec1, &mut vec2)
            } else {
                (&mut vec2, &mut vec1)
            };

            let map: Vec<i32> = looper.iter().filter_map(|node| {
                if let Some(node_value) = node {
                    let node_value_borrowed = node_value.borrow();
                    child_inserter.push(node_value_borrowed.left.clone());
                    child_inserter.push(node_value_borrowed.right.clone());
                    Some(node_value_borrowed.val)
                } else {
                    None
                }
            }).collect();

            if !map.is_empty() {
                ret.push(map)
            }

            looper.clear();
            looper_flag = !looper_flag;
        }

        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod test {
    use crate::binary_tree_level_order_traversal::{Solution, TreeNode};
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


        let vec1 = vec![
            vec![3],
            vec![9, 20],
            vec![15, 7]
        ];

        assert_eq!(vec1, Solution::level_order(Some(Rc::new(RefCell::new(node3)))));
    }

    #[test]
    fn test2() {
        let vec1 = vec![
            vec![4]
        ];
        assert_eq!(vec1, Solution::level_order(Some(Rc::new(RefCell::new(TreeNode::new(4))))));
    }
}
