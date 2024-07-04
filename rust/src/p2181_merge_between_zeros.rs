// Definition for singly-linked list.

type List = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: List,
}

pub struct Solution {}

impl Solution {
    pub fn merge_nodes(mut head: List) -> List {
        let mut current = head.take().unwrap();
        let mut tail = &mut head;
        while let Some(mut node) = current.next.take() {
            if node.val == 0 {
                *tail = Some(current);
                tail = &mut tail.as_mut().unwrap().next;
            } else {
                node.val += current.val
            }
            current = node;
        }
        head
    }
}

#[cfg(test)]
pub mod tests {
    impl ListNode {
        fn from(array: &[i32]) -> List {
            if array.len() == 0 {
                None
            } else {
                Some(Box::new(ListNode {
                    val: array[0],
                    next: Self::from(&array[1..]),
                }))
            }
        }

        fn to_array(l: List) -> Vec<i32> {
            match l {
                None => Vec::new(),
                Some(node) => {
                    let mut ret = Vec::new();
                    ret.push(node.val);
                    ret.append(&mut Self::to_array(node.next));
                    ret
                }
            }
        }
    }

    use super::*;
    fn test_eq(input: &[i32], output: &[i32]) {
        assert_eq!(
            ListNode::to_array(Solution::merge_nodes(ListNode::from(&input))),
            output
        );
    }
    #[test]
    fn test() {
        self::test_eq(&[0, 3, 1, 0, 4, 5, 2, 0], &[4, 11]);
    }
}
