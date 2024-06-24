impl Solution {
    /* Induction/inspection shows that we can just go from left to right and
     * determine if a bit needs flipping, and if so, flipping the subarray
     * starting there.  Once there are too few elements left to form another
     * subarray, we just have to check that every remaining element is 1
     *
     * Instead of flipping the next k-1 entries, we just keep track of whether
     * the next entry needs flipping.  This is the same as taking the parity of the
     * last k flip/non-flips (with flip = 1 and non-flip = 0)
     *
     * Since once we've seen an index, we don't need it again, we can reuse this space
     * to store whether or not we made a flip so that we can remove that flip from
     * consideration by just looking up the kth previous element
     *
     * To make the math a bit shorter (or because I misread the question),
     * we flip the entire array at the start (ie we start with current = 1)
     * and solve the opposite problem of making all 0s
     */
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut flips = 0;
        let mut current = 1;
        for i in 0..nums.len() {
            if i >= k {
                // println!("No longer considering flip from {}", i - k);
                current ^= nums[i - k];
            }
            // println!("preflip {}", nums[i]);
            nums[i] ^= current;
            // println!("postflip {}", nums[i]);
            if i > nums.len() - k {
                if nums[i] == 1 {
                    return -1;
                }
            } else {
                flips += nums[i];
                current ^= nums[i];
                // println!("Flipped {}? {}", i, nums[i]);
            }
        }

        flips
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_cases() {
        assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);
        assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 2), -1);
        assert_eq!(
            Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3),
            3
        );
    }
}
