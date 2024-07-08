impl Solution {
    /*
     * I don't see any obvious recursions besides brute force.  The input
     * limits of n <= 15 suggests we should just count directly.  15!
     * is still over 1.3 trillion, but we can restrict recursion to only
     * when the permutation so far is valid.  This does not prune branches
     * where there is no valid way to complete the partially valid permutation
     * but I don't see an easy way to do so.  Just pruning explicitly invalid
     * permutations runs the test suite in 30ms.
     */
    fn count_recursive(n: usize, mut assigned: &mut Vec<usize>, next: usize) -> i32 {
        if next >= n {
            // println!("Found arrangement {:?}", assigned);
            return 1;
        }
        let mut ret = 0;
        for i in 0..n {
            if assigned[i] == 0 {
                if (i + 1) % (next + 1) == 0 || (next + 1) % (i + 1) == 0 {
                    assigned[i] = next + 1;
                    ret += Self::count_recursive(n, &mut assigned, next + 1);
                    assigned[i] = 0;
                }
            }
        }
        ret
    }

    pub fn count_arrangement(n: i32) -> i32 {
        let mut assigned = vec![0; n as usize];
        Self::count_recursive(n as usize, &mut assigned, 0)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_arrangement(1), 1);
        assert_eq!(Solution::count_arrangement(2), 2);
        assert_eq!(Solution::count_arrangement(3), 3);
        assert_eq!(Solution::count_arrangement(4), 8);
        assert_eq!(Solution::count_arrangement(15), 24679);
    }
}
