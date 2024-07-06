impl Solution {
    /*
     * The situation is the same as having 2n - 2 people
     * and passing the pillow in a loop, with the people labeled
     * 1 2 .. n-1 n n-1 .. 2
     */
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let position = time % (2 * n - 2) + 1;
        if position > n {
            2 * n - position
        } else {
            position
        }
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cases() {
        assert_eq!(Solution::pass_the_pillow(4, 5), 2);
        assert_eq!(Solution::pass_the_pillow(3, 2), 3);
        assert_eq!(Solution::pass_the_pillow(18, 38), 5);
        assert_eq!(Solution::pass_the_pillow(1000, 1000), 999);
    }
}
