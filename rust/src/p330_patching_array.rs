/*
 * Idea: Consider the prefix sums.  If an element is more than 1 greater than the prefix sum prior to it,
 * then we cannot reach that number.  We can inductively show that patching in that number makes every
 * number reachable:
 *
 * Assume all numbers <= k are reachable, then if k+1 is patched in, all numbers up to 2k+1 are
 * reachable.  Indeed if m <= 2k+1, m - (k+1) <= k, and so is the sum of some elements.  Adding k+1 reaches m.
 *
 * Clearly we can't patch a number greater than k+1.  If we patch in a number smaller than k+1, the set of
 * reachable numbers is a subset of what would be reachable with k+1, so always picking k+1 gives the minimum
 * number of patches.
 */

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut patches = 0;
        let mut sum = 0;
        let mut nums_iter = nums.into_iter();
        let mut next = nums_iter.next();
        loop {
            let add;
            if next.is_none() || sum < next.unwrap() - 1 {
                add = sum + 1;
                patches += 1;
                // println!("Patching in {}, covering up to {}", add, add as i64 + sum as i64);
            } else {
                add = next.unwrap();
                next = nums_iter.next();
                // println!("Taking in {}, covering up to {}", add, add as i64 + sum as i64);
            }
            if n - sum <= add {
                return patches;
            } else {
                sum += add;
            }
        }
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::min_patches(vec![1, 3], 6), 1);
        assert_eq!(Solution::min_patches(vec![1, 5, 10], 20), 2);
        assert_eq!(Solution::min_patches(vec![1, 2, 2], 5), 0);
        assert_eq!(Solution::min_patches(vec![1, 2, 31, 33], 2147483647), 28);
        assert_eq!(
            Solution::min_patches(
                vec![
                    1 << 0,
                    1 << 1,
                    1 << 2,
                    1 << 3,
                    1 << 4,
                    1 << 5,
                    1 << 6,
                    1 << 7,
                    1 << 8,
                    1 << 9,
                    1 << 10,
                    1 << 11,
                    1 << 12,
                    1 << 13,
                    1 << 14,
                    1 << 15,
                    1 << 16,
                    1 << 17,
                    1 << 18,
                    1 << 19,
                    1 << 20,
                    1 << 21,
                    1 << 22,
                    1 << 23,
                    1 << 24,
                    1 << 25,
                    1 << 26,
                    1 << 27,
                    1 << 28,
                    1 << 29,
                    (1 << 30) - 1
                ],
                i32::MAX
            ),
            1
        )
    }
}
