use std::collections::BinaryHeap;

impl Solution {
    /*
     * Just keeping a heap of elements for which we are still
     * waiting for the next greatest element.  I keep track
     * of the largest element and whether we are on the second
     * pass to know when to stop.  Alternatively we could just
     * loop exactly twice
     */
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![-1; nums.len()];
        let mut searching: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut max = i32::MIN;
        let mut looped = false;
        let mut index = 0;
        loop {
            let current = nums[index];
            while let Some((val, idx)) = searching.peek() {
                if -val >= current {
                    break;
                } else {
                    ret[*idx] = current;
                    searching.pop();
                }
            }
            max = max.max(current);
            if max == current && looped {
                break;
            }
            searching.push((-current, index));
            index += 1;
            if index >= nums.len() {
                index = 0;
                looped = true;
            }
        }
        ret
    }
}

pub struct Solution {}
