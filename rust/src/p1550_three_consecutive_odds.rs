impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        for a in arr {
            if a % 2 == 0 {
                count = 0;
            } else {
                count += 1;
                if count == 3 {
                    return true;
                }
            }
        }
        false
    }
}

pub struct Solution {}