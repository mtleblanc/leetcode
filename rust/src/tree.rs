use std::{cell::RefCell, rc::Rc};

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

    pub fn from_array_indexed(array: &Vec<i32>, index: usize) -> Tree {
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

    pub fn from_array(array: Vec<i32>) -> Tree {
        Self::from_array_indexed(&array, 0)
    }
}

pub type Tree = Option<Rc<RefCell<TreeNode>>>;
