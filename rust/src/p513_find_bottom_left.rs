use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::tree::{Tree, TreeNode};
impl Solution {
    fn dfs(root: Ref<TreeNode>, depth: i32) -> (i32, i32) {
        let left = root
            .left
            .as_ref()
            .map(|cell| Self::dfs(cell.borrow(), depth + 1));
        let right = root
            .right
            .as_ref()
            .map(|cell| Self::dfs(cell.borrow(), depth + 1));
        let this = Some((depth, root.val));
        [left, right, this]
            .into_iter()
            .flatten()
            .min_by_key(|(d, _)| *d)
            .unwrap()
    }

    fn dfs_with_state(root: Ref<TreeNode>, depth: i32, mut max_depth: &mut i32, mut res: &mut i32) {
        if depth > *max_depth {
            *max_depth = depth;
            *res = root.val;
        }
        root.left.as_ref().inspect(|node| {
            Self::dfs_with_state(node.borrow(), depth + 1, &mut max_depth, &mut res)
        });
        root.right.as_ref().inspect(|node| {
            Self::dfs_with_state(node.borrow(), depth + 1, &mut max_depth, &mut res)
        });
    }

    fn bfs(root: Rc<RefCell<TreeNode>>) -> i32 {
        use std::collections::VecDeque;
        let mut current_row = VecDeque::new();
        let mut next_row = VecDeque::new();
        next_row.push_back(root);
        let mut first = 0;
        while !next_row.is_empty() {
            std::mem::swap(&mut next_row, &mut current_row);
            first = current_row[0].borrow().val;
            while let Some(node) = current_row.pop_front() {
                let cell = node.borrow();
                [cell.left.clone(), cell.right.clone()]
                    .iter()
                    .flatten()
                    .for_each(|node| next_row.push_back(node.clone()));
            }
        }
        first
    }

    pub fn find_bottom_left_value(root: Tree) -> i32 {
        Self::bfs(root.unwrap().clone())
    }

    pub fn find_bottom_left_value_dfs(root: Tree) -> i32 {
        Self::dfs(root.unwrap().borrow(), 0).1
    }

    pub fn find_bottom_left_value_dfs_state(root: Tree) -> i32 {
        let mut res = 0;
        let mut max_depth = -1;
        Self::dfs_with_state(root.unwrap().borrow(), 0, &mut max_depth, &mut res);
        res
    }
}

pub struct Solution {}
