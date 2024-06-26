use std::cell::RefCell;
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;
/**
 * Flatten the tree into an array by doing an in-order traversal, then build a balanced tree directly
 */
impl Solution {
    // Consumes root.  Could also use &Node and leave in tact
    fn to_sorted(root: Node, order: &mut Vec<i32>) {
        if let Some(node_ref) = root {
            let mut tn = node_ref.borrow_mut();
            Self::to_sorted(tn.left.take(), order);
            order.push(tn.val);
            Self::to_sorted(tn.right.take(), order);
        }
    }

    // Imbalances go to the left, could also have pivot = (sorted.len - 1) / 2 to favor right
    fn from_sorted(sorted: &[i32]) -> Node {
        if sorted.len() == 0 {
            return None;
        }
        let pivot = sorted.len() / 2;
        Some(Rc::new(RefCell::new(TreeNode {
            val: sorted[pivot],
            left: Self::from_sorted(&sorted[..pivot]),
            right: Self::from_sorted(&sorted[1 + pivot..]),
        })))
    }

    pub fn balance_bst(root: Node) -> Node {
        let mut sorted = Vec::new();
        Self::to_sorted(root, &mut sorted);
        Self::from_sorted(&sorted)
    }
}

pub struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
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
mod tests {
    use super::*;
    #[test]
    fn to_array() {
        let tree =
            TreeNode::from_array(vec![1, -1, 2, -1, -1, -1, 3, -1, -1, -1, -1, -1, -1, -1, 4]);
        println!("Tree {:?}", tree);
        let mut sorted = Vec::new();
        Solution::to_sorted(tree, &mut sorted);
        assert_eq!(sorted, vec![1, 2, 3, 4]);
    }
}
