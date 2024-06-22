impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let odd_counts = nums.into_iter().scan(0, |sum, i| {
            *sum += (i % 2) as usize;
            Some(*sum)
        });
        let counts = odd_counts.fold(vec![1], |mut counts, i| {
            while counts.len() < i + 1 {
                counts.push(0);
            }
            counts.last_mut().map(|x| *x = *x + 1);
            counts
        });
        if counts.len() < k + 1 {
            return 0;
        }
        let mut total = 0;
        for i in k..counts.len() {
            total += counts[i] * counts[i - k];
        }
        total
    }
}
pub struct Solution {}
