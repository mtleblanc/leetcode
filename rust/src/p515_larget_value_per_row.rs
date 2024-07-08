use crate::tree::Tree;

impl Solution {
    fn bfs(root: Tree) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut current_row = VecDeque::new();
        let mut next_row = VecDeque::new();
        if let Some(root) = root {
            next_row.push_back(root);
        }
        let mut ret = Vec::new();
        while !next_row.is_empty() {
            std::mem::swap(&mut next_row, &mut current_row);
            let mut max = i32::MIN;
            while let Some(node) = current_row.pop_front() {
                let cell = node.borrow();
                max = max.max(cell.val);
                [cell.left.clone(), cell.right.clone()]
                    .iter()
                    .flatten()
                    .for_each(|node| next_row.push_back(node.clone()));
            }
            ret.push(max);
        }
        ret
    }
    pub fn largest_values(root: Tree) -> Vec<i32> {
        Self::bfs(root)
    }
}

pub struct Solution {}
