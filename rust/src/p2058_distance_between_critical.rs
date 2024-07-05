// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn nodes_between_critical_points(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut min = i32::MAX;
        let mut max = -1;
        let mut total = 0;
        let mut current = 0;
        let mut direction = Ordering::Equal;
        let mut seen = false;
        let Some(first) = head else {
            return vec![-1, -1];
        };
        head = first.next;
        let mut prev = first.val;
        while let Some(mut node) = head {
            let new_direction = node.val.cmp(&prev);
            current += 1;
            total += 1;
            if direction != Ordering::Equal
                && new_direction != Ordering::Equal
                && direction != new_direction
            {
                if !seen {
                    seen = true;
                    total = 0;
                } else {
                    min = min.min(current);
                    max = max.max(total);
                }
                current = 0;
            }
            prev = node.val;
            direction = new_direction;
            head = node.next.take();
        }
        if min == i32::MAX {
            return vec![-1, -1];
        }
        vec![min, max]
    }
}
