impl Solution {
    /*
     * Each machine must pass some number of dresses
     * left (l[k] below) and some number right (r[k]),
     * which can be calculated in a single pass comparing
     * prefix sums of the actual and average values.
     *
     * I claim that the machines can be balanced in the max
     * of the sums of these values, and can be done simply
     * by greedily moving dresses whenever possible.
     *
     * Idea: calculate how many times a machine must move
     * dresses both left and right, and show that making
     * such a move a) decreases the calculated values, and
     * b) is always possible for the machine with the most
     * dresses to move.
     *
     * Proof:
     * Define s[k] to be the prefix sum of net movement
     * required (ie sum (avg - machine[k])).  Then
     * define l[k] as the positive values in s[k], offset
     * by 1 (that is l[k] = 0 if s[k-1] < 0, s[k-1] otherwise)
     * and r[k] as the opposite of the negative values with
     * no offset (r[k] = -s[k] if s[k] < 0)
     *
     * Now suppose l[k] > 0.  Then if we move a dress
     * from machine k to machine k-1, then s[k] remains
     * unchanged (as the dress is still in the prefix)
     * and s[k-1] decreases by 1, and s[k-2] and beyond
     * remain the same (as the dress remains out of the
     * prefix).  Since l[k] = s[k-1] > 0 before the move,
     * we still have s[k-1] >= 0 after the move.  Thus
     * r[k-1] = 0 both before and after.  Therefore
     * moving a dress left from a machine k with l[k] > 0
     * decreases l[k] and leaves the remaining l and r values
     * unchanged.
     *
     * Likewise, moving a dress right from machine k
     * with r[k] > 0 increases s[k] by 1 and leaves
     * the others unchanged.  s[k] = r[k] < 0 beforehand
     * and so s[k] <= 0 after, and so likewise only r[k]
     * decreases by 1 and other l, r remain unchanged.
     *
     * Our moves will be greedy, moving a dress left
     * from machine k if l[k] > 0 and it has dresses,
     * otherwise to the right if r[k] > 0 and it has
     * dresses, and doing nothing if it has no dresses
     * or if both l[k] and r[k] are 0.
     *
     * By the observations above, any machine that moves
     * a dress in this manner will decrease one of l[k]
     * or r[k], and no l[k] or r[k] will ever increase.
     *
     * We finish by induction, assume we can balance the
     * machine in m moves if the maximum of l[k] + r[k] is m,
     * (which is trivially true if m = 0).  Now if the
     * maximum is m+1, perform the greedy move.  As long
     * as each machine achieving that maximum has a dress
     * then it's total will decrease to m.
     *
     * Suppose machine k has l[k] + r[k] = m + 1, but has
     * 0 dresses. Then s[k] = a + s[k-1].  If s[k] >= 0,
     * we have l[k+1] = s[k], r[k] = 0.  Then
     * s[k-1] = l[k] = m + 1, and so l[k] = a + m + 1,
     * contradicting the maximal value being m + 1.
     *
     * If s[k] < 0, s[k-1] = s[k] - a < 0, and so l[k] = 0
     * and r[k] = m + 1.  Then r[k-1] = m + 1 + a, again
     * a contradiction and completing the proof.
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
        // [-1,2,-1]
        // [-1,1,0]
        // [0,1,0]
        // [0,1,0]
        assert_eq!(Solution::find_min_moves(vec![0, 2, 0]), -1);
        assert_eq!(Solution::find_min_moves(vec![0, 0, 11, 5]), 8);
        // [-4,-4,7,1]
        // [-4,-8,-1,0]
        // [0,4,8,1]
        // [0,0,0,0]
        assert_eq!(Solution::find_min_moves(vec![9, 1, 8, 8, 9]), 4);
        //  [2,-6,1,1,2]
        //  [2,-4,-3,-2,0]
        //  [2,0,0,0,0]
        //  [0,0,4,3,2]
    }
}
