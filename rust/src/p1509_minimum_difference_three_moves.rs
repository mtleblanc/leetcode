impl Solution {
    /*
     * After moving, the largest and smallest must be among the original
     * largest 4 and the original smallest 4, so these are all that matter.
     * We could run faster by just keeping track of these 8 extremal elements
     * and get a linear time, but adding the logarithmic factor of just sorting
     * still runs in 13ms over the test set.
     *
     * Having the 8 extremal elements, we can make any 3 of them non-extremal
     * (say by setting them to the same value as the 4th smallest), and such
     * a move cannot increase the difference between max and min.  So we can
     * be greedy and pick up to 3 from either end.  So we just have to find
     * the best difference between 0th and n-4th, 1st and n-3rd, etc.
     */
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 5 {
            return 0;
        }
        nums.sort_unstable();
        nums[nums.len() - 4..]
            .iter()
            .zip(nums.iter())
            .map(|(&a, &b)| a - b)
            .min()
            .unwrap()
    }
}

pub struct Solution {}
