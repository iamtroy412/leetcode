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
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root_node = root.as_ref().unwrap().borrow();
        let right_node = root_node.right.as_ref().unwrap().borrow();
        let left_node = root_node.left.as_ref().unwrap().borrow();

        // If the child nodes (left and right) equal the root node
        // return true, else return false
        right_node.val + left_node.val == root_node.val
    }
}
