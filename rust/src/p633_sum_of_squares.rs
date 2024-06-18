impl Solution {
    fn i_sqrt(c: i32) -> i32 {
        (c as f64).sqrt() as i32
    }
    pub fn judge_square_sum(c: i32) -> bool {
        let mut b = Self::i_sqrt(c);
        let mut a = 0;
        while a <= b {
            let trial = c - a * a - b * b;
            if trial == 0 {
                return true;
            }
            if trial < 0 {
                b -= 1;
            }
            if trial > 0 {
                a += 1;
            }
        }
        false
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::judge_square_sum(5), true);
        assert_eq!(Solution::judge_square_sum(3), false);
        assert_eq!(Solution::judge_square_sum(2), true);
        assert_eq!(Solution::judge_square_sum(6), false);
        assert_eq!(Solution::judge_square_sum(2147483600), true);
    }
}
