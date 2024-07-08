impl Solution {
    /*
     * For each index we calculate how many dresses have
     * to move left of it and how many dresses have to move
     * right.  The machine only does work if these numbers
     * are positive (negatives means dresses are provided to it)
     *
     * I claim that the largest amount of work for a single machine
     * answers the question with a sketch of a proof as follows.
     *
     * The idea is that if on a move a machine needs to pass a dress
     * but is empty, then there must be a machine next to it that
     * has more work.  Suppose wolog that the machine needs to pass
     * to the right.  Since there are 0 dresses currently in the
     * machine, it cannot need to pass both left and rigth, so its
     * work is only what must be passed right.
     * Then this means there is above average number
     * of total dresses to the left of it (inclusive).  But it has
     * 0 dresses, so the machine immediately to its left has
     * work at least one average higher than this machine.
     */
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let count = machines.len() as i32;
        let total: i32 = machines.iter().sum();
        if total % count != 0 {
            return -1;
        }
        let target = total / count;
        machines
            .into_iter()
            .scan(0, |incoming, x| {
                let outgoing = *incoming + x - target;
                let pass_back = (-*incoming).max(0);
                let pass_forward = outgoing.max(0);
                *incoming = outgoing;
                Some(pass_back + pass_forward)
            })
            .max()
            .unwrap()
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_min_moves(vec![1, 0, 5]), 3);
        assert_eq!(Solution::find_min_moves(vec![0, 3, 0]), 2);
        assert_eq!(Solution::find_min_moves(vec![0, 2, 0]), -1);
        assert_eq!(Solution::find_min_moves(vec![0, 0, 11, 5]), 8);
        assert_eq!(Solution::find_min_moves(vec![9, 1, 8, 8, 9]), 4);
    }
}
