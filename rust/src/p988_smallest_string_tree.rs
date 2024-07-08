use std::{cmp::Ordering, collections::VecDeque};

use crate::tree::Tree;
/*
 * All three solutions are the same concept, just doing a dfs and mutating smallest
 * directly at each leaf.  I assumed the Vec implementation would be fastest, but
 * it appears to be String.  They are all roughly equal however.  I also tried
 * Vec/VecDequeu<u8> and <i32>.  In each case I preload smallest with a value
 * guaranteed to be larger than any leaf in order to avoid testing for empty
 * 
 * I am assuming that rev() is just a jump to the last index on vectors and strings
 * and not a linear stack.  This belief is buoyed by rev only being on DoubleEndedIterator,
 * depending on the test suite, we'd probably see VecDeque much faster if that were
 * not the case
 */
impl Solution {
    fn dfs_with_string(root: &Tree, smallest: &mut String, current: &mut String) {
        let Some(tree) = root else {
            return;
        };
        let this = tree.borrow();
        current.push((this.val as u8 + 97) as char);
        Self::dfs_with_string(&this.left, smallest, current);
        Self::dfs_with_string(&this.right, smallest, current);
        if this.left == None && this.right == None {
            if current.chars().rev().cmp(smallest.chars().rev()) == Ordering::Less {
                *smallest = current.clone();
            }
        }
        current.pop();
    }

    fn dfs_with_vec(root: &Tree, smallest: &mut Vec<char>, current: &mut Vec<char>) {
        let Some(tree) = root else {
            return;
        };
        let this = tree.borrow();
        current.push((this.val as u8 + 'a' as u8) as char);
        Self::dfs_with_vec(&this.left, smallest, current);
        Self::dfs_with_vec(&this.right, smallest, current);
        if this.left == None && this.right == None {
            if current.iter().rev().cmp(smallest.iter().rev()) == Ordering::Less {
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

    pub fn smallest_from_leaf_string(root: Tree) -> String {
        let mut smallest = String::new();
        smallest.push(char::MAX);
        let mut current = String::new();
        Self::dfs_with_string(&root, &mut smallest, &mut current);
        String::from_iter(smallest.chars().rev())
    }

    pub fn smallest_from_leaf_deque(root: Tree) -> String {
        let mut smallest = VecDeque::new();
        smallest.push_front(char::MAX);
        let mut current = VecDeque::new();
        Self::dfs_with_dequeue(&root, &mut smallest, &mut current);
        String::from_iter(smallest)
    }

    pub fn smallest_from_leaf_vec(root: Tree) -> String {
        let mut smallest = Vec::new();
        smallest.push(char::MAX);
        let mut current = Vec::new();
        Self::dfs_with_vec(&root, &mut smallest, &mut current);
        String::from_iter(smallest.into_iter().rev())
    }

    pub fn smallest_from_leaf(root: Tree) -> String {
        Self::smallest_from_leaf_string(root)
    }
}

pub struct Solution {}
