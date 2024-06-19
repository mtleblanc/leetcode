impl Solution {
    fn can_make(bloom_day: &Vec<i32>, m: i32, k: i32, d: i32) -> bool {
        let days = bloom_day.iter().map(|i| *i <= d);
        let mut found = 0;
        let mut current_count = 0;
        for i in days.into_iter() {
            if i {
                current_count += 1;
                if current_count == k {
                    found += 1;
                    if found == m {
                        return true;
                    }
                    current_count = 0;
                }
            } else {
                current_count = 0;
            }
        }
        false
    }
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let mut hi = *bloom_day.iter().max().unwrap_or(&i32::MAX);
        let mut lo = 0;
        while hi > lo {
            let mid = (hi + lo)/2;
            if Self::can_make(&bloom_day, m, k, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        if Self::can_make(&bloom_day, m, k, hi) {
            hi
        } else {
            -1
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
        assert_eq!(Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
    }
}
