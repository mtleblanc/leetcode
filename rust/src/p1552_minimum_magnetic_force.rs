impl Solution {
    fn can_make(position: &Vec<i32>, m: i32, force: i32) -> bool {
        let mut remaining = m - 1;
        let mut last = *position.first().unwrap();
        // we could do this faster by doing a binary search for the next number
        // but it isn't a huge speedup since it's only called lg(max - min) times
        for &i in position.iter().skip(1) {
            if i - last >= force {
                last = i;
                remaining -= 1;
                if remaining == 0 {
                    return true;
                }
            }
        }
        false
    }

    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();
        let mut hi = *position.iter().max().unwrap() - *position.iter().min().unwrap() + 1;
        let mut lo = 1;
        while hi > lo {
            let mid = (hi + lo) / 2;
            if !Self::can_make(&position, m, mid) {
                // println!("Can't do {}", mid);
                hi = mid;
            } else {
                // println!("Can do {}", mid);
                lo = mid + 1;
            }
        }
        hi - 1
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::max_distance(vec![1, 2, 3, 4, 7], 3), 3);
        assert_eq!(
            Solution::max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2),
            999999999
        );
    }
}
