use crate::tree::{Tree, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /**
     * Just a simple in (reverse) order traversal updating in place
     */
    fn bst_to_gst_with_presum(root: &Tree, sum: &mut i32) {
        if let Some(tn) = root.as_ref() {
            let mut n = tn.borrow_mut();
            Self::bst_to_gst_with_presum(&n.right, sum);
            n.val += *sum;
            *sum = n.val;
            Self::bst_to_gst_with_presum(&n.left, sum);
        }
    }
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::bst_to_gst_with_presum(&root, &mut 0);
        root
    }
}

pub struct Solution {}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn from_array() {
        let tree = TreeNode::from_array(vec![4, 1, 6, 0, 2, 5, -1]);
        assert!(tree.is_some_and(|n| n.borrow().val == 4
            && n.borrow()
                .left
                .as_ref()
                .is_some_and(|n| n.borrow().val == 1)
            && n.borrow()
                .right
                .as_ref()
                .is_some_and(|n| n.borrow().val == 6 && n.borrow().right.is_none())));
    }

    #[test]
    fn p1038() {
        let tree = TreeNode::from_array(vec![4, 1, 6, 0, 2, 5, 7, -1, -1, -1, 3, -1, -1, -1, 8]);
        let expected = TreeNode::from_array(vec![
            30, 36, 21, 36, 35, 26, 15, -1, -1, -1, 33, -1, -1, -1, 8,
        ]);
        assert_eq!(Solution::bst_to_gst(tree), expected);
    }
}
