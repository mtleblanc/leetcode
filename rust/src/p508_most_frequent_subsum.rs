use std::collections::HashMap;

use crate::tree::Tree;
/*
 * Just a dfs calculating subtree sums and tracking them in a map
 *
 * Once the map is calculated, the max elements are extracted in
 * 2 passes
 */
impl Solution {
    fn sum(root: &Tree, mut counts: &mut HashMap<i32, i32>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let sum = node.val
                    + Self::sum(&node.left, &mut counts)
                    + Self::sum(&node.right, &mut counts);
                *counts.entry(sum).or_default() += 1;
                sum
            }
        }
    }
    pub fn find_frequent_tree_sum(root: Tree) -> Vec<i32> {
        let mut sums = HashMap::new();
        Self::sum(&root, &mut sums);
        let max = *sums.values().max().unwrap();
        sums.into_iter()
            .filter(|(_, count)| *count == max)
            .map(|(num, _)| num)
            .collect()
    }
}

pub struct Solution {}
