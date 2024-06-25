use std::cell::RefCell;
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    /**
     * Just a simple in (reverse) order traversal updating in place
     */
    fn bst_to_gst_with_presum(root: &Node, sum: &mut i32) {
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

    fn from_array_indexed(array: &Vec<i32>, index: usize) -> Option<Rc<RefCell<Self>>> {
        if index >= array.len() {
            return None;
        }
        let val = array[index];
        if val < 0 {
            return None;
        }
        let mut node = Self::new(val);
        node.left = Self::from_array_indexed(&array, index * 2 + 1);
        node.right = Self::from_array_indexed(&array, index * 2 + 2);
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn from_array(array: Vec<i32>) -> Option<Rc<RefCell<Self>>> {
        Self::from_array_indexed(&array, 0)
    }
}

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
