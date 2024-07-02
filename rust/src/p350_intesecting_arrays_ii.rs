impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        use std::cmp::Ordering::*;
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut n1 = nums1.into_iter().peekable();
        nums2.retain(|&x| {
            while let Some(y) = n1.peek() {
                match x.cmp(y) {
                    Less => return false,
                    Equal => {
                        _ = n1.next();
                        return true;
                    }
                    Greater => _ = n1.next(),
                }
            }
            false
        });
        nums2
    }

    pub fn intersect_pass1(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut n1 = nums1.into_iter();
        let mut n2 = nums2.into_iter();
        let mut ret = Vec::new();
        let mut next1 = n1.next();
        let mut next2 = n2.next();
        while let Some(v1) = next1 {
            if let Some(v2) = next2 {
                if v1 == v2 {
                    ret.push(v1);
                    next1 = n1.next();
                    next2 = n2.next();
                } else if v1 < v2 {
                    next1 = n1.next();
                } else {
                    next2 = n2.next();
                }
            } else {
                break;
            }
        }
        ret
    }
}

pub struct Solution {}
