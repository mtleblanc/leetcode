impl Solution {
    /*
     * This is basically just Pascal's triangle with max instead of plus
     *
     * Alternatively it's a 2d DP problem.
     *
     * Either way, we just reuse the existing array to store the minimum
     * health (actually the negative of this value) needed once the knight
     * reaches that square.  This can never be less than 1, and is at most
     * the lesser of the value in down or right, minus (actually plus since
     * we are storing negatives) the original value of the square.
     *
     * We could calculate precice bounds for x in the inner loop, but
     * the code runs in 1ms on the test suite so it isn't needed.  It would
     * change the code to O(mn) from O(m(m+n)), so it could matter if n >> m
     *
     * I use i32::MIN for cells outside the grid instead of doing bounds
     * checking.  Alternatively we could also pattern match on the options.
     */
    pub fn calculate_minimum_hp(mut dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        dungeon[m - 1][n - 1] = (dungeon[m - 1][n - 1] - 1).min(-1);
        for diagonal in (0..(m + n - 2)).rev() {
            for x in 0..m {
                if x > diagonal {
                    break;
                }
                let y = diagonal - x;
                if y >= n {
                    continue;
                }
                let right = dungeon
                    .get(x + 1)
                    .and_then(|v| v.get(y))
                    .unwrap_or(&i32::MIN);
                let down = dungeon
                    .get(x)
                    .and_then(|v| v.get(y + 1))
                    .unwrap_or(&i32::MIN);
                let here = right.max(down) + dungeon[x][y];
                dungeon[x][y] = here.min(-1);
            }
        }
        -dungeon[0][0]
    }
}

pub struct Solution {}
