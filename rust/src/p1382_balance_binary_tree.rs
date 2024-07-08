use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::{Tree, TreeNode};
/**
 * Flatten the tree into an array by doing an in-order traversal, then build a balanced tree directly
 */
impl Solution {
    // Consumes root.  Could also use &Node and leave in tact
    fn to_sorted(root: Tree, order: &mut Vec<i32>) {
        if let Some(node_ref) = root {
            let mut tn = node_ref.borrow_mut();
            Self::to_sorted(tn.left.take(), order);
            order.push(tn.val);
            Self::to_sorted(tn.right.take(), order);
        }
    }

    // Imbalances go to the left, could also have pivot = (sorted.len - 1) / 2 to favor right
    fn from_sorted(sorted: &[i32]) -> Tree {
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

    pub fn balance_bst(root: Tree) -> Tree {
        let mut sorted = Vec::new();
        Self::to_sorted(root, &mut sorted);
        Self::from_sorted(&sorted)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;

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
