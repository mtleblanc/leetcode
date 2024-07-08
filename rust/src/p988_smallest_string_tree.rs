use std::{cmp::Ordering, collections::VecDeque};

use crate::tree::Tree;
impl Solution {
    fn dfs(root: &Tree, smallest: &mut Vec<char>, current: &mut Vec<char>) {
        let Some(tree) = root else {
            return;
        };
        let this = tree.borrow();
        current.push((this.val as u8 + 'a' as u8) as char);
        Self::dfs(&this.left, smallest, current);
        Self::dfs(&this.right, smallest, current);
        if this.left == None && this.right == None {
            if current.iter().rev().cmp(smallest.iter()) == Ordering::Less {
                *smallest = current.clone().to_vec();
            }
        }
        current.pop();
    }

    fn dfs_with_dequeue(root: &Tree, smallest: &mut VecDeque<char>, current: &mut VecDeque<char>) {
        let Some(tree) = root else {
            return;
        };
        let this = tree.borrow();
        current.push_front((this.val as u8 + 'a' as u8) as char);
        Self::dfs_with_dequeue(&this.left, smallest, current);
        Self::dfs_with_dequeue(&this.right, smallest, current);
        if this.left == None && this.right == None {
            if current < smallest {
                *smallest = current.clone();
            }
        }
        current.pop_front();
    }

    pub fn smallest_from_leaf_deque(root: Tree) -> String {
        let mut smallest = VecDeque::new();
        smallest.push_front(char::MAX);
        let mut current = VecDeque::new();
        Self::dfs_with_dequeue(&root, &mut smallest, &mut current);
        String::from_iter(smallest)
    }

    pub fn smallest_from_leaf(root: Tree) -> String {
        let mut smallest = Vec::new();
        smallest.push(char::MAX);
        let mut current = Vec::new();
        Self::dfs(&root, &mut smallest, &mut current);
        String::from_iter(smallest.into_iter().rev())
    }
}

pub struct Solution {}
